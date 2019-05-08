var N=null,E="",T="t",U="u",searchIndex={};
var R=["tower_h2","result","Error","Connect","Handshake","handshake","option","reason","Connection","Background","connection","stream_id","streamid","to_string","try_from","borrow_mut","try_into","type_id","poll_data","size_hint","sizehint","poll_trailers","is_end_stream","borrow","typeid","tower_h2::client","poll_ready","into_future","to_owned","clone_into","string","tower_h2::server","formatter","description","Returns the error if the error is an io::Error","HttpService","ConnectError","HandshakeError","ResponseFuture","RecvBody","ConnectFuture"];

searchIndex[R[0]]={"doc":E,"i":[[3,R[2],R[0],"Represents HTTP/2.0 operation errors.",N,N],[3,"Reason",E,"HTTP/2.0 error codes.",N,N],[8,"Body",E,"Trait representing a streaming body of a Request or…",N,N],[16,"Data",E,"Values yielded by the `Body`.",0,N],[16,R[2],E,"The error type this `BufStream` might generate.",0,N],[10,R[18],E,"Attempt to pull out the next data buffer of this stream.",0,[[["self"]],[R[1],["async"]]]],[11,R[19],E,"Returns the bounds on the remaining length of the stream.",0,[[["self"]],[R[20]]]],[10,R[21],E,"Poll for an optional single `HeaderMap` of trailers.",0,[[["self"]],[R[1],["async"]]]],[11,R[22],E,"Returns `true` when the end of stream has been reached.",0,[[["self"]],["bool"]]],[8,R[35],E,"An HTTP service",N,N],[16,"ResponseBody",E,"Response payload.",1,N],[16,R[2],E,"Errors produced by the service.",1,N],[16,"Future",E,"The future response value.",1,N],[10,R[26],E,"Returns `Ready` when the service is able to process…",1,[[["self"]],[R[1],["async"]]]],[10,"call",E,"Process the request and return the response asynchronously.",1,N],[11,"into_service",E,"Wrap the HttpService so that it implements…",1,[[["self"]],["intoservice"]]],[11,"as_service",E,"Same as `into_service` but operates on an HttpService…",1,[[["self"]],["asservice"]]],[3,"NoBody",E,E,N,N],[3,R[39],E,"Allows a stream to be read from the remote.",N,N],[3,"Data",E,E,N,N],[0,"client",E,E,N,N],[3,R[9],R[25],"Task that performs background tasks for a client.",N,N],[3,R[3],E,"Establishes an H2 client connection.",N,N],[3,R[40],E,"Completes with a Connection when the H2 connection has…",N,N],[3,R[8],E,"Exposes a request/response API on an h2 client connection..",N,N],[3,R[4],E,"In progress HTTP/2.0 client handshake.",N,N],[3,R[38],E,"Drives the sending of a request (and its body) until a…",N,N],[3,R[2],E,"Errors produced by client `Connection` calls.",N,N],[4,R[36],E,"Error produced when establishing an H2 client connection.",N,N],[13,R[3],E,"An error occurred when attempting to establish the…",2,N],[13,R[4],E,"An error occurred while performing the HTTP/2.0 handshake.",2,N],[4,R[37],E,"Error produced when performing an HTTP/2.0 handshake.",N,N],[13,"Proto",E,"An error occurred when attempting to perform the HTTP/2.0…",3,N],[13,"Execute",E,"An error occured when attempting to execute a worker task",3,N],[11,"new",E,"Create a new `Connect`.",4,[[["c"],["builder"],["e"]],["self"]]],[11,R[5],E,"Perform the HTTP/2.0 handshake, yielding a `Connection` on…",5,[[[T],["e"]],[R[5]]]],[11,R[11],E,"Returns the stream ID of the response stream, or `None` if…",6,[[["self"]],[R[6],[R[12]]]]],[11,R[7],E,E,7,[[["self"]],[R[6],[R[7]]]]],[0,"server",R[0],E,N,N],[3,"Server",R[31],"Attaches service implementations to h2 connections.",N,N],[3,R[8],E,"Drives connection-level I/O .",N,N],[3,R[9],E,"Task used to process requests",N,N],[4,R[2],E,"Error produced by a `Connection`.",N,N],[13,R[4],E,"Error produced during the HTTP/2.0 handshake.",8,N],[13,"Protocol",E,"Error produced by the HTTP/2.0 stream",8,N],[13,"NewService",E,"Error produced when obtaining the service",8,N],[13,"Service",E,"Error produced by the service",8,N],[13,"Execute",E,"Error produced when attempting to spawn a task",8,N],[8,"Modify",E,"Modify a received request",N,N],[10,"modify",E,"Modify a request before calling the service.",9,[[["self"],["request"]]]],[11,"new",E,E,10,[[["s"],["builder"],["e"]],["self"]]],[11,"serve",E,"Produces a future that is satisfied once the h2 connection…",10,[[["self"],[T]],[R[10]]]],[11,"serve_modified",E,E,10,[[["self"],[T],["f"]],[R[10]]]],[11,"graceful_shutdown",E,"Start an HTTP2 graceful shutdown.",11,[[["self"]]]],[11,R[11],R[0],"Returns the stream ID of the received stream, or `None` if…",12,[[["self"]],[R[12]]]],[11,R[13],E,E,13,[[["self"]],[R[30]]]],[11,"from",E,E,13,[[[T]],[T]]],[11,"into",E,E,13,[[["self"]],[U]]],[11,R[14],E,E,13,[[[U]],[R[1]]]],[11,R[23],E,E,13,[[["self"]],[T]]],[11,R[17],E,E,13,[[["self"]],[R[24]]]],[11,R[15],E,E,13,[[["self"]],[T]]],[11,R[16],E,E,13,[[["self"]],[R[1]]]],[11,R[13],E,E,14,[[["self"]],[R[30]]]],[11,"from",E,E,14,[[[T]],[T]]],[11,"into",E,E,14,[[["self"]],[U]]],[11,R[28],E,E,14,[[["self"]],[T]]],[11,R[29],E,E,14,N],[11,R[14],E,E,14,[[[U]],[R[1]]]],[11,R[23],E,E,14,[[["self"]],[T]]],[11,R[17],E,E,14,[[["self"]],[R[24]]]],[11,R[15],E,E,14,[[["self"]],[T]]],[11,R[16],E,E,14,[[["self"]],[R[1]]]],[11,"equivalent",E,E,14,[[["self"],["k"]],["bool"]]],[11,"from",E,E,15,[[[T]],[T]]],[11,"into",E,E,15,[[["self"]],[U]]],[11,R[14],E,E,15,[[[U]],[R[1]]]],[11,R[23],E,E,15,[[["self"]],[T]]],[11,R[17],E,E,15,[[["self"]],[R[24]]]],[11,R[15],E,E,15,[[["self"]],[T]]],[11,R[16],E,E,15,[[["self"]],[R[1]]]],[11,R[18],E,E,15,[[["self"]],[R[1],["async"]]]],[11,R[19],E,E,15,[[["self"]],[R[20]]]],[11,R[21],E,E,15,[[["self"]],[R[1],["async"]]]],[11,R[22],E,E,15,[[["self"]],["bool"]]],[11,"from",E,E,12,[[[T]],[T]]],[11,"into",E,E,12,[[["self"]],[U]]],[11,R[14],E,E,12,[[[U]],[R[1]]]],[11,R[23],E,E,12,[[["self"]],[T]]],[11,R[17],E,E,12,[[["self"]],[R[24]]]],[11,R[15],E,E,12,[[["self"]],[T]]],[11,R[16],E,E,12,[[["self"]],[R[1]]]],[11,R[18],E,E,12,[[["self"]],[R[1],["async"]]]],[11,R[19],E,E,12,[[["self"]],[R[20]]]],[11,R[21],E,E,12,[[["self"]],[R[1],["async"]]]],[11,R[22],E,E,12,[[["self"]],["bool"]]],[11,"from",E,E,16,[[[T]],[T]]],[11,"into",E,E,16,[[["self"]],[U]]],[11,R[14],E,E,16,[[[U]],[R[1]]]],[11,R[23],E,E,16,[[["self"]],[T]]],[11,R[17],E,E,16,[[["self"]],[R[24]]]],[11,R[15],E,E,16,[[["self"]],[T]]],[11,R[16],E,E,16,[[["self"]],[R[1]]]],[11,"into_buf",E,E,16,[[["self"]],[T]]],[11,"from",R[25],E,17,[[[T]],[T]]],[11,"into",E,E,17,[[["self"]],[U]]],[11,R[14],E,E,17,[[[U]],[R[1]]]],[11,R[23],E,E,17,[[["self"]],[T]]],[11,R[17],E,E,17,[[["self"]],[R[24]]]],[11,R[15],E,E,17,[[["self"]],[T]]],[11,R[16],E,E,17,[[["self"]],[R[1]]]],[11,R[27],E,E,17,[[["self"]],["f"]]],[11,"from",E,E,4,[[[T]],[T]]],[11,"into",E,E,4,[[["self"]],[U]]],[11,R[14],E,E,4,[[[U]],[R[1]]]],[11,R[23],E,E,4,[[["self"]],[T]]],[11,R[17],E,E,4,[[["self"]],[R[24]]]],[11,R[15],E,E,4,[[["self"]],[T]]],[11,R[16],E,E,4,[[["self"]],[R[1]]]],[11,R[26],E,E,4,[[["self"]],[R[1],["async"]]]],[11,"make_service",E,E,4,N],[11,"from",E,E,18,[[[T]],[T]]],[11,"into",E,E,18,[[["self"]],[U]]],[11,R[14],E,E,18,[[[U]],[R[1]]]],[11,R[23],E,E,18,[[["self"]],[T]]],[11,R[17],E,E,18,[[["self"]],[R[24]]]],[11,R[15],E,E,18,[[["self"]],[T]]],[11,R[16],E,E,18,[[["self"]],[R[1]]]],[11,R[27],E,E,18,[[["self"]],["f"]]],[11,"from",E,E,5,[[[T]],[T]]],[11,"into",E,E,5,[[["self"]],[U]]],[11,R[28],E,E,5,[[["self"]],[T]]],[11,R[29],E,E,5,N],[11,R[14],E,E,5,[[[U]],[R[1]]]],[11,R[23],E,E,5,[[["self"]],[T]]],[11,R[17],E,E,5,[[["self"]],[R[24]]]],[11,R[15],E,E,5,[[["self"]],[T]]],[11,R[16],E,E,5,[[["self"]],[R[1]]]],[11,R[26],E,E,5,[[["self"]],[R[1],["async"]]]],[11,"call",E,E,5,N],[11,"from",E,E,19,[[[T]],[T]]],[11,"into",E,E,19,[[["self"]],[U]]],[11,R[14],E,E,19,[[[U]],[R[1]]]],[11,R[23],E,E,19,[[["self"]],[T]]],[11,R[17],E,E,19,[[["self"]],[R[24]]]],[11,R[15],E,E,19,[[["self"]],[T]]],[11,R[16],E,E,19,[[["self"]],[R[1]]]],[11,R[27],E,E,19,[[["self"]],["f"]]],[11,"from",E,E,6,[[[T]],[T]]],[11,"into",E,E,6,[[["self"]],[U]]],[11,R[14],E,E,6,[[[U]],[R[1]]]],[11,R[23],E,E,6,[[["self"]],[T]]],[11,R[17],E,E,6,[[["self"]],[R[24]]]],[11,R[15],E,E,6,[[["self"]],[T]]],[11,R[16],E,E,6,[[["self"]],[R[1]]]],[11,R[27],E,E,6,[[["self"]],["f"]]],[11,R[13],E,E,7,[[["self"]],[R[30]]]],[11,"from",E,E,7,[[[T]],[T]]],[11,"into",E,E,7,[[["self"]],[U]]],[11,R[14],E,E,7,[[[U]],[R[1]]]],[11,R[23],E,E,7,[[["self"]],[T]]],[11,R[17],E,E,7,[[["self"]],[R[24]]]],[11,R[15],E,E,7,[[["self"]],[T]]],[11,R[16],E,E,7,[[["self"]],[R[1]]]],[11,R[13],E,E,2,[[["self"]],[R[30]]]],[11,"from",E,E,2,[[[T]],[T]]],[11,"into",E,E,2,[[["self"]],[U]]],[11,R[14],E,E,2,[[[U]],[R[1]]]],[11,R[23],E,E,2,[[["self"]],[T]]],[11,R[17],E,E,2,[[["self"]],[R[24]]]],[11,R[15],E,E,2,[[["self"]],[T]]],[11,R[16],E,E,2,[[["self"]],[R[1]]]],[11,R[13],E,E,3,[[["self"]],[R[30]]]],[11,"from",E,E,3,[[[T]],[T]]],[11,"into",E,E,3,[[["self"]],[U]]],[11,R[14],E,E,3,[[[U]],[R[1]]]],[11,R[23],E,E,3,[[["self"]],[T]]],[11,R[17],E,E,3,[[["self"]],[R[24]]]],[11,R[15],E,E,3,[[["self"]],[T]]],[11,R[16],E,E,3,[[["self"]],[R[1]]]],[11,"from",R[31],E,10,[[[T]],[T]]],[11,"into",E,E,10,[[["self"]],[U]]],[11,R[28],E,E,10,[[["self"]],[T]]],[11,R[29],E,E,10,N],[11,R[14],E,E,10,[[[U]],[R[1]]]],[11,R[23],E,E,10,[[["self"]],[T]]],[11,R[17],E,E,10,[[["self"]],[R[24]]]],[11,R[15],E,E,10,[[["self"]],[T]]],[11,R[16],E,E,10,[[["self"]],[R[1]]]],[11,"from",E,E,11,[[[T]],[T]]],[11,"into",E,E,11,[[["self"]],[U]]],[11,R[14],E,E,11,[[[U]],[R[1]]]],[11,R[23],E,E,11,[[["self"]],[T]]],[11,R[17],E,E,11,[[["self"]],[R[24]]]],[11,R[15],E,E,11,[[["self"]],[T]]],[11,R[16],E,E,11,[[["self"]],[R[1]]]],[11,R[27],E,E,11,[[["self"]],["f"]]],[11,"from",E,E,20,[[[T]],[T]]],[11,"into",E,E,20,[[["self"]],[U]]],[11,R[14],E,E,20,[[[U]],[R[1]]]],[11,R[23],E,E,20,[[["self"]],[T]]],[11,R[17],E,E,20,[[["self"]],[R[24]]]],[11,R[15],E,E,20,[[["self"]],[T]]],[11,R[16],E,E,20,[[["self"]],[R[1]]]],[11,R[27],E,E,20,[[["self"]],["f"]]],[11,R[13],E,E,8,[[["self"]],[R[30]]]],[11,"from",E,E,8,[[[T]],[T]]],[11,"into",E,E,8,[[["self"]],[U]]],[11,R[14],E,E,8,[[[U]],[R[1]]]],[11,R[23],E,E,8,[[["self"]],[T]]],[11,R[17],E,E,8,[[["self"]],[R[24]]]],[11,R[15],E,E,8,[[["self"]],[T]]],[11,R[16],E,E,8,[[["self"]],[R[1]]]],[11,"fmt",R[0],E,13,[[["self"],[R[32]]],[R[1],["error"]]]],[11,"fmt",E,E,14,[[["self"],[R[32]]],[R[1],["error"]]]],[11,"from",E,E,13,[[["senderror"]],["error"]]],[11,"from",E,E,14,[[["u32"]],[R[7]]]],[11,"from",E,E,13,[[[R[7]]],["error"]]],[11,"from",E,E,13,[[["usererror"]],["error"]]],[11,"from",E,E,13,[[["error"]],["error"]]],[11,"from",E,E,13,[[["error"]],["error"]]],[11,"eq",E,E,14,[[["self"],[R[7]]],["bool"]]],[11,"ne",E,E,14,[[["self"],[R[7]]],["bool"]]],[11,"fmt",E,E,13,[[["self"],[R[32]]],[R[1],["error"]]]],[11,"fmt",E,E,14,[[["self"],[R[32]]],[R[1],["error"]]]],[11,"clone",E,E,14,[[["self"]],[R[7]]]],[11,R[33],E,E,13,[[["self"]],["str"]]],[11,"default",E,E,15,[[],["nobody"]]],[11,"clone",R[25],E,5,[[["self"]],["self"]]],[11,"clone",R[31],E,10,[[["self"]],["self"]]],[11,"from",R[25],E,7,[[["error"]],["self"]]],[11,"from",E,E,7,[[[R[7]]],["self"]]],[11,"from",E,E,3,[[["error"]],["self"]]],[11,"fmt",E,E,2,[[["self"],[R[32]]],[R[1]]]],[11,"fmt",E,E,7,[[["self"],[R[32]]],[R[1]]]],[11,"fmt",E,E,3,[[["self"],[R[32]]],[R[1]]]],[11,"fmt",R[31],E,8,[[["self"],[R[32]]],[R[1]]]],[11,"fmt",R[25],E,2,[[["self"],[R[32]]],[R[1]]]],[11,"fmt",E,E,7,[[["self"],[R[32]]],[R[1]]]],[11,"fmt",E,E,3,[[["self"],[R[32]]],[R[1]]]],[11,"fmt",R[31],E,8,[[["self"],[R[32]]],[R[1]]]],[11,"fmt",R[0],E,15,[[["self"],[R[32]]],[R[1]]]],[11,"fmt",E,E,12,[[["self"],[R[32]]],[R[1]]]],[11,"fmt",E,E,16,[[["self"],[R[32]]],[R[1]]]],[11,R[33],R[25],E,2,[[["self"]],["str"]]],[11,"cause",E,E,2,[[["self"]],[R[6],["error"]]]],[11,"cause",E,E,7,[[["self"]],[R[6],["error"]]]],[11,R[33],E,E,7,[[["self"]],["str"]]],[11,"cause",E,E,3,[[["self"]],[R[6],["error"]]]],[11,R[33],E,E,3,[[["self"]],["str"]]],[11,"cause",R[31],E,8,[[["self"]],[R[6],["error"]]]],[11,R[33],E,E,8,[[["self"]],["str"]]],[11,"remaining",R[0],E,16,[[["self"]],["usize"]]],[11,"bytes",E,E,16,N],[11,"advance",E,E,16,[[["self"],["usize"]]]],[11,"poll",R[25],E,17,[[["self"]],["poll"]]],[11,"poll",E,E,18,[[["self"]],["poll"]]],[11,"poll",E,E,6,[[["self"]],["poll"]]],[11,"poll",E,E,19,[[["self"]],["poll"]]],[11,"poll",R[31],E,11,[[["self"]],["poll"]]],[11,"poll",E,E,20,[[["self"]],["poll"]]],[11,R[22],R[0],E,15,[[["self"]],["bool"]]],[11,R[18],E,E,15,[[["self"]],["poll",[R[6],"error"]]]],[11,R[21],E,E,15,[[["self"]],["poll",[R[6],"error"]]]],[11,R[22],E,E,12,[[["self"]],["bool"]]],[11,R[18],E,E,12,[[["self"]],["poll",[R[6],"error"]]]],[11,R[21],E,E,12,[[["self"]],["poll",[R[6],"error"]]]],[11,R[26],R[25],E,4,[[["self"]],["poll"]]],[11,"call",E,"Obtains a Connection on a single plaintext h2 connection…",4,N],[11,R[26],E,E,5,[[["self"]],["poll"]]],[11,"call",E,E,5,N],[11,R[7],R[0],"If the error was caused by the remote peer, the error…",13,[[["self"]],[R[6],[R[7]]]]],[11,"is_io",E,"Returns the true if the error is an io::Error",13,[[["self"]],["bool"]]],[11,"get_io",E,R[34],13,[[["self"]],[R[6],["error"]]]],[11,"into_io",E,R[34],13,[[["self"]],[R[6],["error"]]]],[18,"NO_ERROR",E,"The associated condition is not a result of an error.",14,N],[18,"PROTOCOL_ERROR",E,"The endpoint detected an unspecific protocol error.",14,N],[18,"INTERNAL_ERROR",E,"The endpoint encountered an unexpected internal error.",14,N],[18,"FLOW_CONTROL_ERROR",E,"The endpoint detected that its peer violated the…",14,N],[18,"SETTINGS_TIMEOUT",E,"The endpoint sent a SETTINGS frame but did not receive a…",14,N],[18,"STREAM_CLOSED",E,"The endpoint received a frame after a stream was…",14,N],[18,"FRAME_SIZE_ERROR",E,"The endpoint received a frame with an invalid size.",14,N],[18,"REFUSED_STREAM",E,"The endpoint refused the stream prior to performing any…",14,N],[18,"CANCEL",E,"Used by the endpoint to indicate that the stream is no…",14,N],[18,"COMPRESSION_ERROR",E,"The endpoint is unable to maintain the header compression…",14,N],[18,"CONNECT_ERROR",E,"The connection established in response to a CONNECT…",14,N],[18,"ENHANCE_YOUR_CALM",E,"The endpoint detected that its peer is exhibiting a…",14,N],[18,"INADEQUATE_SECURITY",E,"The underlying transport has properties that do not meet…",14,N],[18,"HTTP_1_1_REQUIRED",E,"The endpoint requires that HTTP/1.1 be used instead of…",14,N],[11,R[33],E,"Get a string description of the error code.",14,[[["self"]],["str"]]]],"p":[[8,"Body"],[8,R[35]],[4,R[36]],[4,R[37]],[3,R[3]],[3,R[8]],[3,R[38]],[3,R[2]],[4,R[2]],[8,"Modify"],[3,"Server"],[3,R[8]],[3,R[39]],[3,R[2]],[3,"Reason"],[3,"NoBody"],[3,"Data"],[3,R[9]],[3,R[40]],[3,R[4]],[3,R[9]]]};
initSearch(searchIndex);addSearchOptions(searchIndex);