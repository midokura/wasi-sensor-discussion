<h1><a id="sensing"></a>World sensing</h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi_buffer_pool_data_types"><code>wasi:buffer-pool/data-types</code></a></li>
<li>interface <a href="#wasi_sensor_property"><code>wasi:sensor/property</code></a></li>
<li>interface <a href="#wasi_sensor_sensor"><code>wasi:sensor/sensor</code></a></li>
<li>interface <a href="#wasi_io_poll_0_2_0"><code>wasi:io/poll@0.2.0</code></a></li>
<li>interface <a href="#wasi_buffer_pool_buffer_pool"><code>wasi:buffer-pool/buffer-pool</code></a></li>
</ul>
</li>
<li>Exports:
<ul>
<li>interface <a href="#wasi_sensor_interface"><code>wasi:sensor/interface</code></a></li>
</ul>
</li>
</ul>
<h2><a id="wasi_buffer_pool_data_types"></a>Import interface wasi:buffer-pool/data-types</h2>
<p>WASI Sensor is an Sensor abstraction API</p>
<hr />
<h3>Types</h3>
<h4><a id="vector3f"></a><code>record vector3f</code></h4>
<p>sensor data type</p>
<h5>Record Fields</h5>
<ul>
<li><a id="vector3f.x"></a><code>x</code>: <code>f32</code></li>
<li><a id="vector3f.y"></a><code>y</code>: <code>f32</code></li>
<li><a id="vector3f.z"></a><code>z</code>: <code>f32</code></li>
</ul>
<h4><a id="quaternion_f"></a><code>record quaternion-f</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="quaternion_f.x"></a><code>x</code>: <code>f32</code></li>
<li><a id="quaternion_f.y"></a><code>y</code>: <code>f32</code></li>
<li><a id="quaternion_f.z"></a><code>z</code>: <code>f32</code></li>
<li><a id="quaternion_f.w"></a><code>w</code>: <code>f32</code></li>
</ul>
<h4><a id="pixel_format"></a><code>enum pixel-format</code></h4>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a id="pixel_format.gray"></a><code>gray</code></p>
<p>grayscale, bpp=8
</li>
<li>
<p><a id="pixel_format.rgb24"></a><code>rgb24</code></p>
<p>r,g,b bpp=24
</li>
<li>
<p><a id="pixel_format.bgr24"></a><code>bgr24</code></p>
<p>b,g,r bpp=24
</li>
<li>
<p><a id="pixel_format.argb32"></a><code>argb32</code></p>
<p>a,r,g,b bpp=32
</li>
<li>
<p><a id="pixel_format.abgr32"></a><code>abgr32</code></p>
<p>a,b,g,r bpp=32
</li>
<li>
<p><a id="pixel_format.yuy2"></a><code>yuy2</code></p>
<p>YUV422 (Y1,Cb,Y2,Cr) bpp=16
</li>
<li>
<p><a id="pixel_format.mjpeg"></a><code>mjpeg</code></p>
<p>Motion JPEG
</li>
</ul>
<h4><a id="dimension"></a><code>record dimension</code></h4>
<h5>Record Fields</h5>
<ul>
<li>
<p><a id="dimension.width"></a><code>width</code>: <code>u32</code></p>
<p>Image width.
</li>
<li>
<p><a id="dimension.height"></a><code>height</code>: <code>u32</code></p>
<p>Image height.
</li>
<li>
<p><a id="dimension.stride_bytes"></a><code>stride-bytes</code>: <code>u32</code></p>
<p>Image stride.
0 for compressed formats like mjpeg.
</li>
<li>
<p><a id="dimension.pixel_format"></a><a href="#pixel_format"><code>pixel-format</code></a>: <a href="#pixel_format"><a href="#pixel_format"><code>pixel-format</code></a></a></p>
<p>The format of a pixel.
</li>
</ul>
<h4><a id="image"></a><code>record image</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="image.dimension"></a><a href="#dimension"><code>dimension</code></a>: <a href="#dimension"><a href="#dimension"><code>dimension</code></a></a></li>
<li><a id="image.payload"></a><code>payload</code>: list&lt;<code>u8</code>&gt;</li>
</ul>
<h4><a id="depth"></a><code>record depth</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="depth.payload"></a><code>payload</code>: list&lt;<code>u8</code>&gt;<p>dimension of depth image is updated later here
</li>
</ul>
<h4><a id="data_type"></a><code>variant data-type</code></h4>
<h5>Variant Cases</h5>
<ul>
<li><a id="data_type.image"></a><a href="#image"><code>image</code></a>: <a href="#image"><a href="#image"><code>image</code></a></a></li>
</ul>
<h2><a id="wasi_sensor_property"></a>Import interface wasi:sensor/property</h2>
<hr />
<h3>Types</h3>
<h4><a id="dimension"></a><code>type dimension</code></h4>
<p><a href="#dimension"><a href="#dimension"><code>dimension</code></a></a></p>
<p>
#### <a id="fraction"></a>`record fraction`
<h5>Record Fields</h5>
<ul>
<li><a id="fraction.numerator"></a><code>numerator</code>: <code>u32</code></li>
<li><a id="fraction.denominator"></a><code>denominator</code>: <code>u32</code></li>
</ul>
<h4><a id="property_key"></a><code>enum property-key</code></h4>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a id="property_key.sampling_rate"></a><code>sampling-rate</code></p>
<p>The number of samples in a second. (fraction)
Eg. frame rate for image sensors.
</li>
<li>
<p><a id="property_key.dimension"></a><a href="#dimension"><code>dimension</code></a></p>
</li>
</ul>
<h4><a id="property_value"></a><code>variant property-value</code></h4>
<h5>Variant Cases</h5>
<ul>
<li><a id="property_value.fraction"></a><a href="#fraction"><code>fraction</code></a>: <a href="#fraction"><a href="#fraction"><code>fraction</code></a></a></li>
<li><a id="property_value.dimension"></a><a href="#dimension"><code>dimension</code></a>: <a href="#dimension"><a href="#dimension"><code>dimension</code></a></a></li>
</ul>
<h2><a id="wasi_sensor_sensor"></a>Import interface wasi:sensor/sensor</h2>
<hr />
<h3>Types</h3>
<h4><a id="property_key"></a><code>type property-key</code></h4>
<p><a href="#property_key"><a href="#property_key"><code>property-key</code></a></a></p>
<p>
#### <a id="property_value"></a>`type property-value`
[`property-value`](#property_value)
<p>
#### <a id="device_error"></a>`enum device-error`
<h5>Enum Cases</h5>
<ul>
<li><a id="device_error.not_found"></a><code>not-found</code></li>
<li><a id="device_error.invalid_argument"></a><code>invalid-argument</code></li>
<li><a id="device_error.resource_exhausted"></a><code>resource-exhausted</code></li>
<li><a id="device_error.permission_denied"></a><code>permission-denied</code></li>
<li><a id="device_error.busy"></a><code>busy</code></li>
<li><a id="device_error.timeout"></a><code>timeout</code></li>
<li><a id="device_error.cancelled"></a><code>cancelled</code></li>
<li><a id="device_error.aborted"></a><code>aborted</code></li>
<li><a id="device_error.already_exists"></a><code>already-exists</code></li>
<li><a id="device_error.invalid_operation"></a><code>invalid-operation</code></li>
<li><a id="device_error.out_of_range"></a><code>out-of-range</code></li>
<li><a id="device_error.data_loss"></a><code>data-loss</code></li>
<li><a id="device_error.hardware_error"></a><code>hardware-error</code></li>
<li><a id="device_error.not_supported"></a><code>not-supported</code></li>
<li><a id="device_error.unknown"></a><code>unknown</code></li>
</ul>
<h4><a id="device"></a><code>resource device</code></h4>
<h2>Sensor device</h2>
<h3>Functions</h3>
<h4><a id="static_device_open"></a><code>[static]device.open: func</code></h4>
<p>open the device.
this might power on the device.</p>
<h5>Params</h5>
<ul>
<li><a id="static_device_open.name"></a><code>name</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_device_open.0"></a> result&lt;own&lt;<a href="#device"><a href="#device"><code>device</code></a></a>&gt;, <a href="#device_error"><a href="#device_error"><code>device-error</code></a></a>&gt;</li>
</ul>
<h4><a id="static_device_list_names"></a><code>[static]device.list-names: func</code></h4>
<p>get a list of names of devices available on the system.</p>
<h5>Return values</h5>
<ul>
<li><a id="static_device_list_names.0"></a> result&lt;list&lt;<code>string</code>&gt;, <a href="#device_error"><a href="#device_error"><code>device-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_device_start"></a><code>[method]device.start: func</code></h4>
<p>start sending the data to buffer</p>
<h5>Params</h5>
<ul>
<li><a id="method_device_start.self"></a><code>self</code>: borrow&lt;<a href="#device"><a href="#device"><code>device</code></a></a>&gt;</li>
<li><a id="method_device_start.buffer_pool"></a><code>buffer-pool</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_device_start.0"></a> result&lt;_, <a href="#device_error"><a href="#device_error"><code>device-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_device_stop"></a><code>[method]device.stop: func</code></h4>
<p>stop sending the data to buffer</p>
<h5>Params</h5>
<ul>
<li><a id="method_device_stop.self"></a><code>self</code>: borrow&lt;<a href="#device"><a href="#device"><code>device</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_device_stop.0"></a> result&lt;_, <a href="#device_error"><a href="#device_error"><code>device-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_device_set_property"></a><code>[method]device.set-property: func</code></h4>
<p>set property</p>
<h5>Params</h5>
<ul>
<li><a id="method_device_set_property.self"></a><code>self</code>: borrow&lt;<a href="#device"><a href="#device"><code>device</code></a></a>&gt;</li>
<li><a id="method_device_set_property.key"></a><code>key</code>: <a href="#property_key"><a href="#property_key"><code>property-key</code></a></a></li>
<li><a id="method_device_set_property.value"></a><code>value</code>: <a href="#property_value"><a href="#property_value"><code>property-value</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_device_set_property.0"></a> result&lt;_, <a href="#device_error"><a href="#device_error"><code>device-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_device_get_property"></a><code>[method]device.get-property: func</code></h4>
<p>get property</p>
<h5>Params</h5>
<ul>
<li><a id="method_device_get_property.self"></a><code>self</code>: borrow&lt;<a href="#device"><a href="#device"><code>device</code></a></a>&gt;</li>
<li><a id="method_device_get_property.property"></a><code>property</code>: <a href="#property_key"><a href="#property_key"><code>property-key</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_device_get_property.0"></a> result&lt;<a href="#property_value"><a href="#property_value"><code>property-value</code></a></a>, <a href="#device_error"><a href="#device_error"><code>device-error</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_io_poll_0_2_0"></a>Import interface wasi:io/poll@0.2.0</h2>
<p>A poll API intended to let users wait for I/O events on multiple handles
at once.</p>
<hr />
<h3>Types</h3>
<h4><a id="pollable"></a><code>resource pollable</code></h4>
<h2><a href="#pollable"><code>pollable</code></a> represents a single I/O event which may be ready, or not.</h2>
<h3>Functions</h3>
<h4><a id="method_pollable_ready"></a><code>[method]pollable.ready: func</code></h4>
<p>Return the readiness of a pollable. This function never blocks.</p>
<p>Returns <code>true</code> when the pollable is ready, and <code>false</code> otherwise.</p>
<h5>Params</h5>
<ul>
<li><a id="method_pollable_ready.self"></a><code>self</code>: borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_pollable_ready.0"></a> <code>bool</code></li>
</ul>
<h4><a id="method_pollable_block"></a><code>[method]pollable.block: func</code></h4>
<p><code>block</code> returns immediately if the pollable is ready, and otherwise
blocks until ready.</p>
<p>This function is equivalent to calling <code>poll.poll</code> on a list
containing only this pollable.</p>
<h5>Params</h5>
<ul>
<li><a id="method_pollable_block.self"></a><code>self</code>: borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="poll"></a><code>poll: func</code></h4>
<p>Poll for completion on a set of pollables.</p>
<p>This function takes a list of pollables, which identify I/O sources of
interest, and waits until one or more of the events is ready for I/O.</p>
<p>The result <code>list&lt;u32&gt;</code> contains one or more indices of handles in the
argument list that is ready for I/O.</p>
<p>If the list contains more elements than can be indexed with a <code>u32</code>
value, this function traps.</p>
<p>A timeout can be implemented by adding a pollable from the
wasi-clocks API to the list.</p>
<p>This function does not return a <code>result</code>; polling in itself does not
do any I/O so it doesn't fail. If any of the I/O sources identified by
the pollables has an error, it is indicated by marking the source as
being reaedy for I/O.</p>
<h5>Params</h5>
<ul>
<li><a id="poll.in"></a><code>in</code>: list&lt;borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="poll.0"></a> list&lt;<code>u32</code>&gt;</li>
</ul>
<h2><a id="wasi_buffer_pool_buffer_pool"></a>Import interface wasi:buffer-pool/buffer-pool</h2>
<p>sensor frame/buffer management I/F</p>
<hr />
<h3>Types</h3>
<h4><a id="pollable"></a><code>type pollable</code></h4>
<p><a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></p>
<p>
#### <a id="data_type"></a>`type data-type`
[`data-type`](#data_type)
<p>
#### <a id="buffer_error"></a>`enum buffer-error`
<h5>Enum Cases</h5>
<ul>
<li><a id="buffer_error.not_found"></a><code>not-found</code></li>
<li><a id="buffer_error.invalid_argument"></a><code>invalid-argument</code></li>
<li><a id="buffer_error.resource_exhausted"></a><code>resource-exhausted</code></li>
<li><a id="buffer_error.permission_denied"></a><code>permission-denied</code></li>
<li><a id="buffer_error.busy"></a><code>busy</code></li>
<li><a id="buffer_error.timeout"></a><code>timeout</code></li>
<li><a id="buffer_error.cancelled"></a><code>cancelled</code></li>
<li><a id="buffer_error.aborted"></a><code>aborted</code></li>
<li><a id="buffer_error.already_exists"></a><code>already-exists</code></li>
<li><a id="buffer_error.invalid_operation"></a><code>invalid-operation</code></li>
<li><a id="buffer_error.out_of_range"></a><code>out-of-range</code></li>
<li><a id="buffer_error.data_loss"></a><code>data-loss</code></li>
<li><a id="buffer_error.hardware_error"></a><code>hardware-error</code></li>
<li><a id="buffer_error.not_supported"></a><code>not-supported</code></li>
<li><a id="buffer_error.unknown"></a><code>unknown</code></li>
</ul>
<h4><a id="size"></a><code>type size</code></h4>
<p><code>u32</code></p>
<p>
#### <a id="timestamp"></a>`type timestamp`
`u64`
<p>timestamp is the elasped time in nanoseconds since a fixed point
in the past. it's supposed to increase monotonically.
<h4><a id="memory"></a><code>resource memory</code></h4>
<h4><a id="frame_data"></a><code>variant frame-data</code></h4>
<h5>Variant Cases</h5>
<ul>
<li>
<p><a id="frame_data.by_value"></a><code>by-value</code>: <a href="#data_type"><a href="#data_type"><code>data-type</code></a></a></p>
<p>data passed by value
</li>
<li>
<p><a id="frame_data.host_memory"></a><code>host-memory</code>: own&lt;<a href="#memory"><a href="#memory"><code>memory</code></a></a>&gt;</p>
<p>a reference to host memory
</li>
</ul>
<h4><a id="frame_info"></a><code>record frame-info</code></h4>
<h5>Record Fields</h5>
<ul>
<li>
<p><a id="frame_info.sequence_number"></a><code>sequence-number</code>: <code>u64</code></p>
<p>sequence number within the pool. it increases monotonically.
a user of this api might observe discontiguous values when some
of frames are discarded within the pool.
</li>
<li>
<p><a id="frame_info.timestamp"></a><a href="#timestamp"><code>timestamp</code></a>: <a href="#timestamp"><a href="#timestamp"><code>timestamp</code></a></a></p>
<p>timestamp of the frame.
usually the time when it was read from the underlying hardware.
</li>
<li>
<p><a id="frame_info.data"></a><code>data</code>: list&lt;<a href="#frame_data"><a href="#frame_data"><code>frame-data</code></a></a>&gt;</p>
<p>1 or more raw-data for this frame.
</li>
</ul>
<h4><a id="buffering_mode"></a><code>enum buffering-mode</code></h4>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a id="buffering_mode.buffering_off"></a><code>buffering-off</code></p>
</li>
<li>
<p><a id="buffering_mode.buffering_discard"></a><code>buffering-discard</code></p>
</li>
<li>
<p><a id="buffering_mode.buffering_overwrite"></a><code>buffering-overwrite</code></p>
<p>< Discard the latest frame. behave like queue
</li>
<li>
<p><a id="buffering_mode.buffering_unlimited"></a><code>buffering-unlimited</code></p>
<p>< Overwrite the oldest frame. behave like ring
</li>
</ul>
<h4><a id="pool_statistics"></a><code>record pool-statistics</code></h4>
<h5>Record Fields</h5>
<ul>
<li><a id="pool_statistics.enqueued"></a><code>enqueued</code>: <code>u64</code></li>
<li><a id="pool_statistics.dropped"></a><code>dropped</code>: <code>u64</code></li>
<li><a id="pool_statistics.dequeued"></a><code>dequeued</code>: <code>u64</code></li>
</ul>
<h4><a id="pool"></a><code>resource pool</code></h4>
<h2>a pool consists of a set of buffers.
the number of buffers in a pool is static.
when data (a frame) comes in from an associated device,
it's stored in one of free buffers.
when a user app request data either by block-read or poll-read,
the oldest frame is returned.
when the user app is done with the frame, it should notify it to
the pool by dropping the frame-info and associated resources
like &quot;memory&quot;.</h2>
<h3>Functions</h3>
<h4><a id="method_memory_address"></a><code>[method]memory.address: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_memory_address.self"></a><code>self</code>: borrow&lt;<a href="#memory"><a href="#memory"><code>memory</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_memory_address.0"></a> <code>u64</code></li>
</ul>
<h4><a id="method_memory_size"></a><code>[method]memory.size: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_memory_size.self"></a><code>self</code>: borrow&lt;<a href="#memory"><a href="#memory"><code>memory</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_memory_size.0"></a> <a href="#size"><a href="#size"><code>size</code></a></a></li>
</ul>
<h4><a id="method_memory_invalidate"></a><code>[method]memory.invalidate: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_memory_invalidate.self"></a><code>self</code>: borrow&lt;<a href="#memory"><a href="#memory"><code>memory</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_memory_invalidate.0"></a> result&lt;_, <a href="#buffer_error"><a href="#buffer_error"><code>buffer-error</code></a></a>&gt;</li>
</ul>
<h4><a id="static_pool_create"></a><code>[static]pool.create: func</code></h4>
<p>create a pool.</p>
<p>size: the max size of each buffer in bytes. if frame-info::data
has exactly one data and its type is data-types::image,
this value controls the max payload size. otherwise, it's
implementation-defined.</p>
<p>buffer-num: the max number of buffers in the pool.
for buffering-discard and buffering-overwrite, this controls
how many frames can be in the pool.
for other buffering modes, this is ignored.</p>
<p>name: the name of the pool. you can use this for device.start().</p>
<h5>Params</h5>
<ul>
<li><a id="static_pool_create.mode"></a><code>mode</code>: <a href="#buffering_mode"><a href="#buffering_mode"><code>buffering-mode</code></a></a></li>
<li><a id="static_pool_create.size"></a><a href="#size"><code>size</code></a>: <code>u32</code></li>
<li><a id="static_pool_create.buffer_num"></a><code>buffer-num</code>: <code>u32</code></li>
<li><a id="static_pool_create.name"></a><code>name</code>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_pool_create.0"></a> result&lt;own&lt;<a href="#pool"><a href="#pool"><code>pool</code></a></a>&gt;, <a href="#buffer_error"><a href="#buffer_error"><code>buffer-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_pool_read_frames"></a><code>[method]pool.read-frames: func</code></h4>
<p>try to read frames.
this function returns 0 frames when</p>
<ul>
<li>max-results = 0</li>
<li>or, no frames are immediately available</li>
</ul>
<h5>Params</h5>
<ul>
<li><a id="method_pool_read_frames.self"></a><code>self</code>: borrow&lt;<a href="#pool"><a href="#pool"><code>pool</code></a></a>&gt;</li>
<li><a id="method_pool_read_frames.max_results"></a><code>max-results</code>: <code>u32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_pool_read_frames.0"></a> result&lt;list&lt;<a href="#frame_info"><a href="#frame_info"><code>frame-info</code></a></a>&gt;, <a href="#buffer_error"><a href="#buffer_error"><code>buffer-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_pool_subscribe"></a><code>[method]pool.subscribe: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_pool_subscribe.self"></a><code>self</code>: borrow&lt;<a href="#pool"><a href="#pool"><code>pool</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_pool_subscribe.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a id="method_pool_get_statistics"></a><code>[method]pool.get-statistics: func</code></h4>
<h5>Params</h5>
<ul>
<li><a id="method_pool_get_statistics.self"></a><code>self</code>: borrow&lt;<a href="#pool"><a href="#pool"><code>pool</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_pool_get_statistics.0"></a> result&lt;<a href="#pool_statistics"><a href="#pool_statistics"><code>pool-statistics</code></a></a>, <a href="#buffer_error"><a href="#buffer_error"><code>buffer-error</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_sensor_interface"></a>Export interface wasi:sensor/interface</h2>
<hr />
<h3>Functions</h3>
<h4><a id="main"></a><code>main: func</code></h4>
<h5>Return values</h5>
<ul>
<li><a id="main.0"></a> result</li>
</ul>
