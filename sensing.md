<h1><a name="sensing">World sensing</a></h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi:buffer_pool_data_types"><code>wasi:buffer-pool/data-types</code></a></li>
<li>interface <a href="#wasi:sensor_property"><code>wasi:sensor/property</code></a></li>
<li>interface <a href="#wasi:sensor_sensor"><code>wasi:sensor/sensor</code></a></li>
<li>interface <a href="#wasi:io_poll_0.2.0"><code>wasi:io/poll@0.2.0</code></a></li>
<li>interface <a href="#wasi:buffer_pool_buffer_pool"><code>wasi:buffer-pool/buffer-pool</code></a></li>
</ul>
</li>
<li>Exports:
<ul>
<li>interface <a href="#wasi:sensor_interface"><code>wasi:sensor/interface</code></a></li>
</ul>
</li>
</ul>
<h2><a name="wasi:buffer_pool_data_types">Import interface wasi:buffer-pool/data-types</a></h2>
<p>WASI Sensor is an Sensor abstraction API</p>
<hr />
<h3>Types</h3>
<h4><a name="vector3f"><code>record vector3f</code></a></h4>
<p>sensor data type</p>
<h5>Record Fields</h5>
<ul>
<li><a name="vector3f.x"><code>x</code></a>: <code>float32</code></li>
<li><a name="vector3f.y"><code>y</code></a>: <code>float32</code></li>
<li><a name="vector3f.z"><code>z</code></a>: <code>float32</code></li>
</ul>
<h4><a name="quaternion_f"><code>record quaternion-f</code></a></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="quaternion_f.x"><code>x</code></a>: <code>float32</code></li>
<li><a name="quaternion_f.y"><code>y</code></a>: <code>float32</code></li>
<li><a name="quaternion_f.z"><code>z</code></a>: <code>float32</code></li>
<li><a name="quaternion_f.w"><code>w</code></a>: <code>float32</code></li>
</ul>
<h4><a name="pixel_format"><code>enum pixel-format</code></a></h4>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a name="pixel_format.gray"><code>gray</code></a></p>
<p>grayscale, bpp=8
</li>
<li>
<p><a name="pixel_format.rgb24"><code>rgb24</code></a></p>
<p>r,g,b bpp=24
</li>
<li>
<p><a name="pixel_format.bgr24"><code>bgr24</code></a></p>
<p>b,g,r bpp=24
</li>
<li>
<p><a name="pixel_format.argb32"><code>argb32</code></a></p>
<p>a,r,g,b bpp=32
</li>
<li>
<p><a name="pixel_format.abgr32"><code>abgr32</code></a></p>
<p>a,b,g,r bpp=32
</li>
<li>
<p><a name="pixel_format.yuy2"><code>yuy2</code></a></p>
<p>YUV422 (Y1,Cb,Y2,Cr) bpp=16
</li>
<li>
<p><a name="pixel_format.mjpeg"><code>mjpeg</code></a></p>
<p>Motion JPEG
</li>
</ul>
<h4><a name="dimension"><code>record dimension</code></a></h4>
<h5>Record Fields</h5>
<ul>
<li>
<p><a name="dimension.width"><code>width</code></a>: <code>u32</code></p>
<p>Image width.
</li>
<li>
<p><a name="dimension.height"><code>height</code></a>: <code>u32</code></p>
<p>Image height.
</li>
<li>
<p><a name="dimension.stride_bytes"><code>stride-bytes</code></a>: <code>u32</code></p>
<p>Image stride.
0 for compressed formats like mjpeg.
</li>
<li>
<p><a name="dimension.pixel_format"><a href="#pixel_format"><code>pixel-format</code></a></a>: <a href="#pixel_format"><a href="#pixel_format"><code>pixel-format</code></a></a></p>
<p>The format of a pixel.
</li>
</ul>
<h4><a name="image"><code>record image</code></a></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="image.dimension"><a href="#dimension"><code>dimension</code></a></a>: <a href="#dimension"><a href="#dimension"><code>dimension</code></a></a></li>
<li><a name="image.payload"><code>payload</code></a>: list&lt;<code>u8</code>&gt;</li>
</ul>
<h4><a name="depth"><code>record depth</code></a></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="depth.payload"><code>payload</code></a>: list&lt;<code>u8</code>&gt;<p>dimension of depth image is updated later here
</li>
</ul>
<h4><a name="data_type"><code>variant data-type</code></a></h4>
<h5>Variant Cases</h5>
<ul>
<li><a name="data_type.image"><a href="#image"><code>image</code></a></a>: <a href="#image"><a href="#image"><code>image</code></a></a></li>
</ul>
<h2><a name="wasi:sensor_property">Import interface wasi:sensor/property</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="dimension"><code>type dimension</code></a></h4>
<p><a href="#dimension"><a href="#dimension"><code>dimension</code></a></a></p>
<p>
#### <a name="fraction">`record fraction`</a>
<h5>Record Fields</h5>
<ul>
<li><a name="fraction.numerator"><code>numerator</code></a>: <code>u32</code></li>
<li><a name="fraction.denominator"><code>denominator</code></a>: <code>u32</code></li>
</ul>
<h4><a name="property_key"><code>enum property-key</code></a></h4>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a name="property_key.sampling_rate"><code>sampling-rate</code></a></p>
<p>The number of samples in a second. (fraction)
Eg. frame rate for image sensors.
</li>
<li>
<p><a name="property_key.dimension"><a href="#dimension"><code>dimension</code></a></a></p>
</li>
</ul>
<h4><a name="property_value"><code>variant property-value</code></a></h4>
<h5>Variant Cases</h5>
<ul>
<li><a name="property_value.fraction"><a href="#fraction"><code>fraction</code></a></a>: <a href="#fraction"><a href="#fraction"><code>fraction</code></a></a></li>
<li><a name="property_value.dimension"><a href="#dimension"><code>dimension</code></a></a>: <a href="#dimension"><a href="#dimension"><code>dimension</code></a></a></li>
</ul>
<h2><a name="wasi:sensor_sensor">Import interface wasi:sensor/sensor</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="property_key"><code>type property-key</code></a></h4>
<p><a href="#property_key"><a href="#property_key"><code>property-key</code></a></a></p>
<p>
#### <a name="property_value">`type property-value`</a>
[`property-value`](#property_value)
<p>
#### <a name="device_error">`enum device-error`</a>
<h5>Enum Cases</h5>
<ul>
<li><a name="device_error.not_found"><code>not-found</code></a></li>
<li><a name="device_error.invalid_argument"><code>invalid-argument</code></a></li>
<li><a name="device_error.resource_exhausted"><code>resource-exhausted</code></a></li>
<li><a name="device_error.permission_denied"><code>permission-denied</code></a></li>
<li><a name="device_error.busy"><code>busy</code></a></li>
<li><a name="device_error.timeout"><code>timeout</code></a></li>
<li><a name="device_error.cancelled"><code>cancelled</code></a></li>
<li><a name="device_error.aborted"><code>aborted</code></a></li>
<li><a name="device_error.already_exists"><code>already-exists</code></a></li>
<li><a name="device_error.invalid_operation"><code>invalid-operation</code></a></li>
<li><a name="device_error.out_of_range"><code>out-of-range</code></a></li>
<li><a name="device_error.data_loss"><code>data-loss</code></a></li>
<li><a name="device_error.hardware_error"><code>hardware-error</code></a></li>
<li><a name="device_error.not_supported"><code>not-supported</code></a></li>
<li><a name="device_error.unknown"><code>unknown</code></a></li>
</ul>
<h4><a name="device"><code>resource device</code></a></h4>
<h2>Sensor device</h2>
<h3>Functions</h3>
<h4><a name="static_device.open"><code>[static]device.open: func</code></a></h4>
<p>open the device.
this might power on the device.</p>
<h5>Params</h5>
<ul>
<li><a name="static_device.open.name"><code>name</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="static_device.open.0"></a> result&lt;own&lt;<a href="#device"><a href="#device"><code>device</code></a></a>&gt;, <a href="#device_error"><a href="#device_error"><code>device-error</code></a></a>&gt;</li>
</ul>
<h4><a name="static_device.list_names"><code>[static]device.list-names: func</code></a></h4>
<p>get a list of names of devices available on the system.</p>
<h5>Return values</h5>
<ul>
<li><a name="static_device.list_names.0"></a> result&lt;list&lt;<code>string</code>&gt;, <a href="#device_error"><a href="#device_error"><code>device-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_device.start"><code>[method]device.start: func</code></a></h4>
<p>start sending the data to buffer</p>
<h5>Params</h5>
<ul>
<li><a name="method_device.start.self"><code>self</code></a>: borrow&lt;<a href="#device"><a href="#device"><code>device</code></a></a>&gt;</li>
<li><a name="method_device.start.buffer_pool"><code>buffer-pool</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_device.start.0"></a> result&lt;_, <a href="#device_error"><a href="#device_error"><code>device-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_device.stop"><code>[method]device.stop: func</code></a></h4>
<p>stop sending the data to buffer</p>
<h5>Params</h5>
<ul>
<li><a name="method_device.stop.self"><code>self</code></a>: borrow&lt;<a href="#device"><a href="#device"><code>device</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_device.stop.0"></a> result&lt;_, <a href="#device_error"><a href="#device_error"><code>device-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_device.set_property"><code>[method]device.set-property: func</code></a></h4>
<p>set property</p>
<h5>Params</h5>
<ul>
<li><a name="method_device.set_property.self"><code>self</code></a>: borrow&lt;<a href="#device"><a href="#device"><code>device</code></a></a>&gt;</li>
<li><a name="method_device.set_property.key"><code>key</code></a>: <a href="#property_key"><a href="#property_key"><code>property-key</code></a></a></li>
<li><a name="method_device.set_property.value"><code>value</code></a>: <a href="#property_value"><a href="#property_value"><code>property-value</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_device.set_property.0"></a> result&lt;_, <a href="#device_error"><a href="#device_error"><code>device-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_device.get_property"><code>[method]device.get-property: func</code></a></h4>
<p>get property</p>
<h5>Params</h5>
<ul>
<li><a name="method_device.get_property.self"><code>self</code></a>: borrow&lt;<a href="#device"><a href="#device"><code>device</code></a></a>&gt;</li>
<li><a name="method_device.get_property.property"><code>property</code></a>: <a href="#property_key"><a href="#property_key"><code>property-key</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_device.get_property.0"></a> result&lt;<a href="#property_value"><a href="#property_value"><code>property-value</code></a></a>, <a href="#device_error"><a href="#device_error"><code>device-error</code></a></a>&gt;</li>
</ul>
<h2><a name="wasi:io_poll_0.2.0">Import interface wasi:io/poll@0.2.0</a></h2>
<p>A poll API intended to let users wait for I/O events on multiple handles
at once.</p>
<hr />
<h3>Types</h3>
<h4><a name="pollable"><code>resource pollable</code></a></h4>
<h2><a href="#pollable"><code>pollable</code></a> represents a single I/O event which may be ready, or not.</h2>
<h3>Functions</h3>
<h4><a name="method_pollable.ready"><code>[method]pollable.ready: func</code></a></h4>
<p>Return the readiness of a pollable. This function never blocks.</p>
<p>Returns <code>true</code> when the pollable is ready, and <code>false</code> otherwise.</p>
<h5>Params</h5>
<ul>
<li><a name="method_pollable.ready.self"><code>self</code></a>: borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_pollable.ready.0"></a> <code>bool</code></li>
</ul>
<h4><a name="method_pollable.block"><code>[method]pollable.block: func</code></a></h4>
<p><code>block</code> returns immediately if the pollable is ready, and otherwise
blocks until ready.</p>
<p>This function is equivalent to calling <code>poll.poll</code> on a list
containing only this pollable.</p>
<h5>Params</h5>
<ul>
<li><a name="method_pollable.block.self"><code>self</code></a>: borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a name="poll"><code>poll: func</code></a></h4>
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
<li><a name="poll.in"><code>in</code></a>: list&lt;borrow&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="poll.0"></a> list&lt;<code>u32</code>&gt;</li>
</ul>
<h2><a name="wasi:buffer_pool_buffer_pool">Import interface wasi:buffer-pool/buffer-pool</a></h2>
<p>sensor frame/buffer management I/F</p>
<hr />
<h3>Types</h3>
<h4><a name="pollable"><code>type pollable</code></a></h4>
<p><a href="#pollable"><a href="#pollable"><code>pollable</code></a></a></p>
<p>
#### <a name="data_type">`type data-type`</a>
[`data-type`](#data_type)
<p>
#### <a name="buffer_error">`enum buffer-error`</a>
<h5>Enum Cases</h5>
<ul>
<li><a name="buffer_error.not_found"><code>not-found</code></a></li>
<li><a name="buffer_error.invalid_argument"><code>invalid-argument</code></a></li>
<li><a name="buffer_error.resource_exhausted"><code>resource-exhausted</code></a></li>
<li><a name="buffer_error.permission_denied"><code>permission-denied</code></a></li>
<li><a name="buffer_error.busy"><code>busy</code></a></li>
<li><a name="buffer_error.timeout"><code>timeout</code></a></li>
<li><a name="buffer_error.cancelled"><code>cancelled</code></a></li>
<li><a name="buffer_error.aborted"><code>aborted</code></a></li>
<li><a name="buffer_error.already_exists"><code>already-exists</code></a></li>
<li><a name="buffer_error.invalid_operation"><code>invalid-operation</code></a></li>
<li><a name="buffer_error.out_of_range"><code>out-of-range</code></a></li>
<li><a name="buffer_error.data_loss"><code>data-loss</code></a></li>
<li><a name="buffer_error.hardware_error"><code>hardware-error</code></a></li>
<li><a name="buffer_error.not_supported"><code>not-supported</code></a></li>
<li><a name="buffer_error.unknown"><code>unknown</code></a></li>
</ul>
<h4><a name="size"><code>type size</code></a></h4>
<p><code>u32</code></p>
<p>
#### <a name="timestamp">`type timestamp`</a>
`u64`
<p>timestamp is the elasped time in nanoseconds since a fixed point
in the past. it's supposed to increase monotonically.
<h4><a name="memory"><code>resource memory</code></a></h4>
<h4><a name="frame_data"><code>variant frame-data</code></a></h4>
<h5>Variant Cases</h5>
<ul>
<li>
<p><a name="frame_data.by_value"><code>by-value</code></a>: <a href="#data_type"><a href="#data_type"><code>data-type</code></a></a></p>
<p>data passed by value
</li>
<li>
<p><a name="frame_data.host_memory"><code>host-memory</code></a>: own&lt;<a href="#memory"><a href="#memory"><code>memory</code></a></a>&gt;</p>
<p>a reference to host memory
</li>
</ul>
<h4><a name="frame_info"><code>record frame-info</code></a></h4>
<h5>Record Fields</h5>
<ul>
<li>
<p><a name="frame_info.sequence_number"><code>sequence-number</code></a>: <code>u64</code></p>
<p>sequence number within the pool. it increases monotonically.
a user of this api might observe discontiguous values when some
of frames are discarded within the pool.
</li>
<li>
<p><a name="frame_info.timestamp"><a href="#timestamp"><code>timestamp</code></a></a>: <a href="#timestamp"><a href="#timestamp"><code>timestamp</code></a></a></p>
<p>timestamp of the frame.
usually the time when it was read from the underlying hardware.
</li>
<li>
<p><a name="frame_info.data"><code>data</code></a>: list&lt;<a href="#frame_data"><a href="#frame_data"><code>frame-data</code></a></a>&gt;</p>
<p>1 or more raw-data for this frame.
</li>
</ul>
<h4><a name="buffering_mode"><code>enum buffering-mode</code></a></h4>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a name="buffering_mode.buffering_off"><code>buffering-off</code></a></p>
</li>
<li>
<p><a name="buffering_mode.buffering_discard"><code>buffering-discard</code></a></p>
</li>
<li>
<p><a name="buffering_mode.buffering_overwrite"><code>buffering-overwrite</code></a></p>
<p>< Discard the latest frame. behave like queue
</li>
<li>
<p><a name="buffering_mode.buffering_unlimited"><code>buffering-unlimited</code></a></p>
<p>< Overwrite the oldest frame. behave like ring
</li>
</ul>
<h4><a name="pool_statistics"><code>record pool-statistics</code></a></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="pool_statistics.enqueued"><code>enqueued</code></a>: <code>u64</code></li>
<li><a name="pool_statistics.dropped"><code>dropped</code></a>: <code>u64</code></li>
<li><a name="pool_statistics.dequeued"><code>dequeued</code></a>: <code>u64</code></li>
</ul>
<h4><a name="pool"><code>resource pool</code></a></h4>
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
<h4><a name="method_memory.address"><code>[method]memory.address: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_memory.address.self"><code>self</code></a>: borrow&lt;<a href="#memory"><a href="#memory"><code>memory</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_memory.address.0"></a> <code>u64</code></li>
</ul>
<h4><a name="method_memory.size"><code>[method]memory.size: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_memory.size.self"><code>self</code></a>: borrow&lt;<a href="#memory"><a href="#memory"><code>memory</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_memory.size.0"></a> <a href="#size"><a href="#size"><code>size</code></a></a></li>
</ul>
<h4><a name="method_memory.invalidate"><code>[method]memory.invalidate: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_memory.invalidate.self"><code>self</code></a>: borrow&lt;<a href="#memory"><a href="#memory"><code>memory</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_memory.invalidate.0"></a> result&lt;_, <a href="#buffer_error"><a href="#buffer_error"><code>buffer-error</code></a></a>&gt;</li>
</ul>
<h4><a name="static_pool.create"><code>[static]pool.create: func</code></a></h4>
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
<li><a name="static_pool.create.mode"><code>mode</code></a>: <a href="#buffering_mode"><a href="#buffering_mode"><code>buffering-mode</code></a></a></li>
<li><a name="static_pool.create.size"><a href="#size"><code>size</code></a></a>: <code>u32</code></li>
<li><a name="static_pool.create.buffer_num"><code>buffer-num</code></a>: <code>u32</code></li>
<li><a name="static_pool.create.name"><code>name</code></a>: <code>string</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="static_pool.create.0"></a> result&lt;own&lt;<a href="#pool"><a href="#pool"><code>pool</code></a></a>&gt;, <a href="#buffer_error"><a href="#buffer_error"><code>buffer-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_pool.read_frames"><code>[method]pool.read-frames: func</code></a></h4>
<p>try to read frames.
this function returns 0 frames when</p>
<ul>
<li>max-results = 0</li>
<li>or, no frames are immediately available</li>
</ul>
<h5>Params</h5>
<ul>
<li><a name="method_pool.read_frames.self"><code>self</code></a>: borrow&lt;<a href="#pool"><a href="#pool"><code>pool</code></a></a>&gt;</li>
<li><a name="method_pool.read_frames.max_results"><code>max-results</code></a>: <code>u32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_pool.read_frames.0"></a> result&lt;list&lt;<a href="#frame_info"><a href="#frame_info"><code>frame-info</code></a></a>&gt;, <a href="#buffer_error"><a href="#buffer_error"><code>buffer-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_pool.subscribe"><code>[method]pool.subscribe: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_pool.subscribe.self"><code>self</code></a>: borrow&lt;<a href="#pool"><a href="#pool"><code>pool</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_pool.subscribe.0"></a> own&lt;<a href="#pollable"><a href="#pollable"><code>pollable</code></a></a>&gt;</li>
</ul>
<h4><a name="method_pool.get_statistics"><code>[method]pool.get-statistics: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_pool.get_statistics.self"><code>self</code></a>: borrow&lt;<a href="#pool"><a href="#pool"><code>pool</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_pool.get_statistics.0"></a> result&lt;<a href="#pool_statistics"><a href="#pool_statistics"><code>pool-statistics</code></a></a>, <a href="#buffer_error"><a href="#buffer_error"><code>buffer-error</code></a></a>&gt;</li>
</ul>
<h2><a name="wasi:sensor_interface">Export interface wasi:sensor/interface</a></h2>
<hr />
<h3>Functions</h3>
<h4><a name="main"><code>main: func</code></a></h4>
<h5>Return values</h5>
<ul>
<li><a name="main.0"></a> result</li>
</ul>
