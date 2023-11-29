#include <errno.h>
#include <inttypes.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include <jpeglib.h>

#include "sensing.h"

#define STRING_LITERAL(str)                                                   \
        {                                                                     \
                .ptr = (uint8_t *)str, .len = sizeof(str) - 1                 \
        }

sensing_string_t device_name = STRING_LITERAL("dummy");
sensing_string_t pool_name = STRING_LITERAL("my-pool");

bool
process_pixel_image(const wasi_buffer_pool_data_types_image_t *image)
{
        const wasi_buffer_pool_data_types_dimension_t *dimension =
                &image->dimension;
        const uint8_t *payload = image->payload.ptr;
        const size_t payload_len = image->payload.len;

        switch (dimension->pixel_format) {
        case WASI_BUFFER_POOL_DATA_TYPES_PIXEL_FORMAT_RGB24:
                break;
        default:
                fprintf(stderr, "unimplemented pixel format %u\n",
                        dimension->pixel_format);
                return false;
        }

        struct timespec tv;
        if (clock_gettime(CLOCK_REALTIME, &tv)) {
                fprintf(stderr, "clock_gettime failed\n");
                return false;
        }
        uintmax_t timestamp_ns =
                (uintmax_t)tv.tv_sec * 1000000000 + tv.tv_nsec;
        char filename[PATH_MAX];
        snprintf(filename, sizeof(filename), "%ju.jpg", timestamp_ns);

        FILE *fp = fopen(filename, "w");
        if (fp == NULL) {
                fprintf(stderr, "failed to open a file %s: %s\n", filename,
                        strerror(errno));
                return false;
        }

        struct jpeg_compress_struct cinfo;
        struct jpeg_error_mgr jerr;
        cinfo.err = jpeg_std_error(&jerr);
        jpeg_create_compress(&cinfo);
        jpeg_stdio_dest(&cinfo, fp);
        cinfo.image_width = dimension->width;
        cinfo.image_height = dimension->height;
        cinfo.in_color_space = JCS_RGB;
        cinfo.input_components = 3;
        jpeg_set_defaults(&cinfo);
        jpeg_start_compress(&cinfo, TRUE);
        uint32_t i;
        for (i = 0; i < dimension->height; i++) {
                /* we assume 8-bit JSAMPLE */
                JSAMPROW row = (JSAMPROW)&payload[dimension->stride_bytes * i];
                jpeg_write_scanlines(&cinfo, &row, 1);
        }
        jpeg_finish_compress(&cinfo);
        jpeg_destroy_compress(&cinfo);
        fclose(fp);
        return true;
}

bool
process_frame_info(wasi_buffer_pool_buffer_pool_frame_info_t *frame)
{
        fprintf(stderr, "sequence_number %" PRIu64 ", timestamp %ju\n",
                frame->sequence_number, (uintmax_t)frame->timestamp);
        size_t i;
        for (i = 0; i < frame->data.len; i++) {
                wasi_buffer_pool_buffer_pool_frame_data_t *data =
                        &frame->data.ptr[i];
                wasi_buffer_pool_data_types_data_type_t *data_type;
                wasi_buffer_pool_data_types_image_t *image;
                switch (data->tag) {
                case WASI_BUFFER_POOL_BUFFER_POOL_FRAME_DATA_BY_VALUE:
                        data_type = &data->val.by_value;
                        switch (data_type->tag) {
                        case WASI_BUFFER_POOL_DATA_TYPES_DATA_TYPE_IMAGE:
                                image = &data_type->val.image;
                                if (!process_pixel_image(image)) {
                                        return false;
                                }
                                break;
                        default:
                                fprintf(stderr,
                                        "unimplemented data-type tag %u\n",
                                        data_type->tag);
                                return false;
                        }
                        break;
                default:
                        fprintf(stderr, "unimplemented frame-data tag %u\n",
                                data->tag);
                        return false;
                }
        }
        return true;
}

bool
exports_wasi_sensor_interface_main()
{
        /*
         * a hack alert!
         * see https://github.com/bytecodealliance/wasmtime/issues/7592
         */
        void __wasm_call_ctors();
        __wasm_call_ctors();

        fprintf(stderr, "C guest started\n");

        wasi_buffer_pool_buffer_pool_buffer_error_t buffer_error;
        sensing_own_pool_t pool;
        if (!wasi_buffer_pool_buffer_pool_static_pool_create(
                    WASI_BUFFER_POOL_BUFFER_POOL_BUFFERING_MODE_BUFFERING_DISCARD,
                    0, 1, &pool_name, &pool, &buffer_error)) {
                fprintf(stderr, "pool.create failed (error %u)\n",
                        (unsigned int)buffer_error);
                return false;
        }
        fprintf(stderr, "pool.create succeeded\n");

        wasi_sensor_sensor_device_error_t device_error;
        sensing_own_device_t device;
        if (!wasi_sensor_sensor_static_device_open(&device_name, &device,
                                                   &device_error)) {
                fprintf(stderr, "device.open failed (error %u)\n",
                        (unsigned int)device_error);
                return false;
        }
        fprintf(stderr, "device.open succeeded\n");

        sensing_borrow_device_t borrowed_device =
                wasi_sensor_sensor_borrow_device(device);
        if (!wasi_sensor_sensor_method_device_start(
                    borrowed_device, &pool_name, &device_error)) {
                fprintf(stderr, "device.start failed (error %u)\n",
                        (unsigned int)device_error);
                return false;
        }
        fprintf(stderr, "device.start succeeded\n");

        sensing_borrow_pool_t borrowed_pool =
                wasi_buffer_pool_buffer_pool_borrow_pool(pool);
        int n = 60;
        int i;
        for (i = 0; i < n; i++) {
                wasi_buffer_pool_buffer_pool_frame_info_t frame;
                if (!wasi_buffer_pool_buffer_pool_method_pool_block_read_frame(
                            borrowed_pool, &frame, &buffer_error)) {
                        fprintf(stderr, "block-read-frame failed (error %u)\n",
                                (unsigned int)buffer_error);
                        return false;
                }
                fprintf(stderr, "got a frame (%u/%u)\n", i + 1, n);
                if (!process_frame_info(&frame)) {
                        return false;
                }
                wasi_buffer_pool_buffer_pool_frame_info_free(&frame);
        }

        fprintf(stderr, "cleaning up\n");
        wasi_sensor_sensor_device_drop_own(device);
        wasi_buffer_pool_buffer_pool_pool_drop_own(pool);

        fprintf(stderr, "succeeded\n");
        return true;
}
