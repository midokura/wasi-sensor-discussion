#include <errno.h>
#include <getopt.h>
#include <inttypes.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include <jpeglib.h>

#include "sensing.h"

#define STRING_LITERAL(str)                                                   \
        {                                                                     \
                .ptr = (uint8_t *)str, .len = sizeof(str) - 1                 \
        }

sensing_string_t pool_name = STRING_LITERAL("my-pool");

static uint32_t
clamp(float f)
{
        if (f < 0.0) {
                return 0;
        }
        if (f > 255.0) {
                return 255;
        }
        return (uint32_t)f;
}

/*
 * a naive implementation. maybe it's better to use fix point arithmetic.
 * https://hk.interaction-lab.org/firewire/yuv.html
 */
static void
yuv_to_rgb(uint32_t y, uint32_t u, uint32_t v, uint32_t *r, uint32_t *g,
           uint32_t *b)
{
        float fy = 1.164 * ((float)y - 16.0);
        float fu = (float)u - 128.0;
        float fv = (float)v - 128.0;
        float fr = fy + 1.596 * fv;
        float fg = fy - 0.293 * fu - 0.813 * fv;
        float fb = fy + 2.018 * fu;
        *r = clamp(fr);
        *g = clamp(fg);
        *b = clamp(fb);
}

static uint8_t *
convert_yuy2_to_rgb(uint32_t width, uint32_t height, uint32_t stride,
                    const uint8_t *yuyv)
{
        uint8_t *rgb = malloc(width * height * 3);
        if (rgb == NULL) {
                return rgb;
        }
        uint8_t *rgbpixel = rgb;
        uint32_t x;
        uint32_t y;
        for (y = 0; y < height; y++) {
                const uint8_t *yuyvpixel = &yuyv[y * stride];
                for (x = 0; x < width; x += 2) {
                        uint32_t y1 = yuyvpixel[0];
                        uint32_t u = yuyvpixel[1];
                        uint32_t y2 = yuyvpixel[2];
                        uint32_t v = yuyvpixel[3];
                        yuyvpixel += 4;

                        uint32_t r;
                        uint32_t g;
                        uint32_t b;
                        yuv_to_rgb(y1, u, v, &r, &g, &b);
                        *rgbpixel++ = r;
                        *rgbpixel++ = g;
                        *rgbpixel++ = b;
                        yuv_to_rgb(y2, u, v, &r, &g, &b);
                        *rgbpixel++ = r;
                        *rgbpixel++ = g;
                        *rgbpixel++ = b;
                }
        }
        return rgb;
}

bool
process_pixel_image(const wasi_buffer_pool_data_types_image_t *image)
{
        const wasi_buffer_pool_data_types_dimension_t *dimension =
                &image->dimension;
        const uint8_t *payload = image->payload.ptr;
        const size_t payload_len = image->payload.len;
        uint8_t *converted = NULL;
        fprintf(stderr, "width %u height %u stride %u len %u\n",
                (int)dimension->width, (int)dimension->height,
                (int)dimension->stride_bytes, (int)payload_len);

        uint32_t stride_bytes;
        switch (dimension->pixel_format) {
        case WASI_BUFFER_POOL_DATA_TYPES_PIXEL_FORMAT_RGB24:
                stride_bytes = dimension->stride_bytes;
                break;
        case WASI_BUFFER_POOL_DATA_TYPES_PIXEL_FORMAT_YUY2:
                converted = convert_yuy2_to_rgb(
                        dimension->width, dimension->height,
                        dimension->stride_bytes, payload);
                if (converted == NULL) {
                        goto fail;
                }
                payload = converted;
                stride_bytes = dimension->width * 3;
                break;
        default:
                fprintf(stderr, "unimplemented pixel format %u\n",
                        dimension->pixel_format);
                goto fail;
        }

        struct timespec tv;
        if (clock_gettime(CLOCK_REALTIME, &tv)) {
                fprintf(stderr, "clock_gettime failed\n");
                goto fail;
        }
        uintmax_t timestamp_ns =
                (uintmax_t)tv.tv_sec * 1000000000 + tv.tv_nsec;
        char filename[PATH_MAX];
        snprintf(filename, sizeof(filename), "%ju.jpg", timestamp_ns);

        FILE *fp = fopen(filename, "w");
        if (fp == NULL) {
                fprintf(stderr, "failed to open a file %s: %s\n", filename,
                        strerror(errno));
                goto fail;
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
                JSAMPROW row = (JSAMPROW)&payload[stride_bytes * i];
                jpeg_write_scanlines(&cinfo, &row, 1);
        }
        jpeg_finish_compress(&cinfo);
        jpeg_destroy_compress(&cinfo);
        fclose(fp);
        free(converted);
        return true;

fail:
        free(converted);
        return false;
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

        const char *sensor = "dummy:dummy";
        {
                uint8_t *argbuf;
                size_t arg_size;
                char **argv;
                size_t argc;
                int ret;
                ret = __wasi_args_sizes_get(&argc, &arg_size);
                if (ret != 0) {
                        fprintf(stderr,
                                "__wasi_args_sizes_get failed with %d\n", ret);
                        return false;
                }
                argv = malloc((argc + 1) * sizeof(char *));
                argbuf = malloc(arg_size);
                if (argv == NULL || argbuf == NULL) {
                        fprintf(stderr, "malloc failed\n");
                        return false;
                }
                memset(argv, 0, (argc + 1) * sizeof(char *));
                ret = __wasi_args_get((uint8_t **)argv, argbuf);
                if (ret != 0) {
                        fprintf(stderr, "__wasi_args_get failed with %d\n",
                                ret);
                        return false;
                }

                size_t i;
                for (i = 0; i < argc; i++) {
                        fprintf(stderr, "argv[%zu] = %s\n", i, argv[i]);
                }

                enum longopt {
                        opt_sensor,
                };
                static const struct option longopts[] = {
                        {
                                "sensor",
                                required_argument,
                                NULL,
                                opt_sensor,
                        },
                        {
                                NULL,
                                0,
                                NULL,
                                0,
                        },
                };
                int longidx;
                while ((ret = getopt_long(argc, argv, "", longopts,
                                          &longidx)) != -1) {
                        switch (ret) {
                        case opt_sensor:
                                sensor = optarg;
                                break;
                        default:
                                fprintf(stderr, "usage error\n");
                                return false;
                        }
                }
        }

        wasi_buffer_pool_buffer_pool_buffer_error_t buffer_error;
        wasi_buffer_pool_buffer_pool_own_pool_t pool;
        if (!wasi_buffer_pool_buffer_pool_static_pool_create(
                    WASI_BUFFER_POOL_BUFFER_POOL_BUFFERING_MODE_BUFFERING_DISCARD,
                    0, 1, &pool_name, &pool, &buffer_error)) {
                fprintf(stderr, "pool.create failed (error %u)\n",
                        (unsigned int)buffer_error);
                return false;
        }
        fprintf(stderr, "pool.create succeeded\n");

        sensing_string_t device_name = {
                .ptr = (uint8_t *)sensor,
                .len = strlen(sensor),
        };
        wasi_sensor_sensor_device_error_t device_error;
        wasi_sensor_sensor_own_device_t device;
        if (!wasi_sensor_sensor_static_device_open(&device_name, &device,
                                                   &device_error)) {
                fprintf(stderr, "device.open failed (error %u)\n",
                        (unsigned int)device_error);
                return false;
        }
        fprintf(stderr, "device.open succeeded\n");

        wasi_sensor_sensor_borrow_device_t borrowed_device =
                wasi_sensor_sensor_borrow_device(device);
        if (!wasi_sensor_sensor_method_device_start(
                    borrowed_device, &pool_name, &device_error)) {
                fprintf(stderr, "device.start failed (error %u)\n",
                        (unsigned int)device_error);
                return false;
        }
        fprintf(stderr, "device.start succeeded\n");

        wasi_buffer_pool_buffer_pool_borrow_pool_t borrowed_pool =
                wasi_buffer_pool_buffer_pool_borrow_pool(pool);
        wasi_io_poll_own_pollable_t poll =
                wasi_buffer_pool_buffer_pool_method_pool_subscribe(
                        borrowed_pool);
        wasi_io_poll_borrow_pollable_t borrowed_poll =
                wasi_io_poll_borrow_pollable(poll);
        int n = 60;
        int i;
        for (i = 0; i < n;) {
                wasi_io_poll_method_pollable_block(
                        borrowed_poll);
                wasi_buffer_pool_buffer_pool_list_frame_info_t frames;
                if (!wasi_buffer_pool_buffer_pool_method_pool_read_frames(
                            borrowed_pool, 1, &frames, &buffer_error)) {
                        fprintf(stderr, "block-read-frame failed (error %u)\n",
                                (unsigned int)buffer_error);
                        return false;
                }
                size_t j;
                for (j = 0; j < frames.len; j++) {
                        wasi_buffer_pool_buffer_pool_frame_info_t *frame =
                                &frames.ptr[j];
                        i++;
                        fprintf(stderr, "got a frame (%u/%u)\n", i, n);
                        if (!process_frame_info(frame)) {
                                return false;
                        }
                }
                wasi_buffer_pool_buffer_pool_list_frame_info_free(&frames);
        }

        wasi_buffer_pool_buffer_pool_pool_statistics_t stats;
        if (!wasi_buffer_pool_buffer_pool_method_pool_get_statistics(
                    borrowed_pool, &stats, &buffer_error)) {
                fprintf(stderr, "get-statistics failed (error %u)\n",
                        (unsigned int)buffer_error);
                return false;
        }
        fprintf(stderr,
                "stats: enqueued=%" PRIu64 " dequeued=%" PRIu64
                " dropped=%" PRIu64 "\n",
                stats.enqueued, stats.dequeued, stats.dropped);

        fprintf(stderr, "cleaning up\n");
        wasi_io_poll_pollable_drop_own(poll);
        wasi_sensor_sensor_device_drop_own(device);
        wasi_buffer_pool_buffer_pool_pool_drop_own(pool);

        fprintf(stderr, "succeeded\n");
        return true;
}
