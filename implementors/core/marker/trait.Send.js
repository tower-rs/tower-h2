(function() {var implementors = {};
implementors["tower_h2"] = [{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"tower_h2/struct.NoBody.html\" title=\"struct tower_h2::NoBody\">NoBody</a>",synthetic:true,types:["tower_h2::body::NoBody"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"tower_h2/struct.RecvBody.html\" title=\"struct tower_h2::RecvBody\">RecvBody</a>",synthetic:true,types:["tower_h2::recv_body::RecvBody"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"tower_h2/struct.Data.html\" title=\"struct tower_h2::Data\">Data</a>",synthetic:true,types:["tower_h2::recv_body::Data"]},{text:"impl&lt;T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"tower_h2/client/struct.Background.html\" title=\"struct tower_h2::client::Background\">Background</a>&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as <a class=\"trait\" href=\"tower_h2/trait.Body.html\" title=\"trait tower_h2::Body\">Body</a>&gt;::<a class=\"type\" href=\"tower_h2/trait.Body.html#associatedtype.Data\" title=\"type tower_h2::Body::Data\">Data</a>: <a class=\"trait\" href=\"https://docs.rs/bytes/0.4.12/bytes/buf/buf/trait.Buf.html\" title=\"trait bytes::buf::buf::Buf\">Buf</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>",synthetic:true,types:["tower_h2::client::background::Background"]},{text:"impl&lt;A, C, E, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"tower_h2/client/struct.Connect.html\" title=\"struct tower_h2::client::Connect\">Connect</a>&lt;A, C, E, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>",synthetic:true,types:["tower_h2::client::connect::Connect"]},{text:"impl&lt;A, C, E, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"tower_h2/client/struct.ConnectFuture.html\" title=\"struct tower_h2::client::ConnectFuture\">ConnectFuture</a>&lt;A, C, E, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;C as <a class=\"trait\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_connection/trait.MakeConnection.html\" title=\"trait tower_util::make_connection::MakeConnection\">MakeConnection</a>&lt;A&gt;&gt;::<a class=\"type\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_connection/trait.MakeConnection.html#associatedtype.Connection\" title=\"type tower_util::make_connection::MakeConnection::Connection\">Connection</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as <a class=\"trait\" href=\"tower_h2/trait.Body.html\" title=\"trait tower_h2::Body\">Body</a>&gt;::<a class=\"type\" href=\"tower_h2/trait.Body.html#associatedtype.Data\" title=\"type tower_h2::Body::Data\">Data</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;C as <a class=\"trait\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_connection/trait.MakeConnection.html\" title=\"trait tower_util::make_connection::MakeConnection\">MakeConnection</a>&lt;A&gt;&gt;::<a class=\"type\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_connection/trait.MakeConnection.html#associatedtype.Future\" title=\"type tower_util::make_connection::MakeConnection::Future\">Future</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>",synthetic:true,types:["tower_h2::client::connect::ConnectFuture"]},{text:"impl&lt;T, E, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"tower_h2/client/struct.Connection.html\" title=\"struct tower_h2::client::Connection\">Connection</a>&lt;T, E, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as <a class=\"trait\" href=\"tower_h2/trait.Body.html\" title=\"trait tower_h2::Body\">Body</a>&gt;::<a class=\"type\" href=\"tower_h2/trait.Body.html#associatedtype.Data\" title=\"type tower_h2::Body::Data\">Data</a>: <a class=\"trait\" href=\"https://docs.rs/bytes/0.4.12/bytes/buf/buf/trait.Buf.html\" title=\"trait bytes::buf::buf::Buf\">Buf</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>",synthetic:true,types:["tower_h2::client::connection::Connection"]},{text:"impl&lt;T, E, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"tower_h2/client/struct.Handshake.html\" title=\"struct tower_h2::client::Handshake\">Handshake</a>&lt;T, E, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as <a class=\"trait\" href=\"tower_h2/trait.Body.html\" title=\"trait tower_h2::Body\">Body</a>&gt;::<a class=\"type\" href=\"tower_h2/trait.Body.html#associatedtype.Data\" title=\"type tower_h2::Body::Data\">Data</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>",synthetic:true,types:["tower_h2::client::connection::Handshake"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"tower_h2/client/struct.ResponseFuture.html\" title=\"struct tower_h2::client::ResponseFuture\">ResponseFuture</a>",synthetic:true,types:["tower_h2::client::connection::ResponseFuture"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"tower_h2/client/struct.Error.html\" title=\"struct tower_h2::client::Error\">Error</a>",synthetic:true,types:["tower_h2::client::connection::Error"]},{text:"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"tower_h2/client/enum.ConnectError.html\" title=\"enum tower_h2::client::ConnectError\">ConnectError</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>",synthetic:true,types:["tower_h2::client::connect::ConnectError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"tower_h2/client/enum.HandshakeError.html\" title=\"enum tower_h2::client::HandshakeError\">HandshakeError</a>",synthetic:true,types:["tower_h2::client::connection::HandshakeError"]},{text:"impl&lt;S, E, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"tower_h2/server/struct.Server.html\" title=\"struct tower_h2::server::Server\">Server</a>&lt;S, E, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>",synthetic:true,types:["tower_h2::server::Server"]},{text:"impl&lt;T, S, E, B, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"tower_h2/server/struct.Connection.html\" title=\"struct tower_h2::server::Connection\">Connection</a>&lt;T, S, E, B, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;B as <a class=\"trait\" href=\"tower_h2/trait.Body.html\" title=\"trait tower_h2::Body\">Body</a>&gt;::<a class=\"type\" href=\"tower_h2/trait.Body.html#associatedtype.Data\" title=\"type tower_h2::Body::Data\">Data</a>: <a class=\"trait\" href=\"https://docs.rs/bytes/0.4.12/bytes/buf/buf/trait.Buf.html\" title=\"trait bytes::buf::buf::Buf\">Buf</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as <a class=\"trait\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_service/trait.MakeService.html\" title=\"trait tower_util::make_service::MakeService\">MakeService</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://docs.rs/http/0.1.17/http/request/struct.Request.html\" title=\"struct http::request::Request\">Request</a>&lt;<a class=\"struct\" href=\"tower_h2/struct.RecvBody.html\" title=\"struct tower_h2::RecvBody\">RecvBody</a>&gt;&gt;&gt;::<a class=\"type\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_service/trait.MakeService.html#associatedtype.Error\" title=\"type tower_util::make_service::MakeService::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as <a class=\"trait\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_service/trait.MakeService.html\" title=\"trait tower_util::make_service::MakeService\">MakeService</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://docs.rs/http/0.1.17/http/request/struct.Request.html\" title=\"struct http::request::Request\">Request</a>&lt;<a class=\"struct\" href=\"tower_h2/struct.RecvBody.html\" title=\"struct tower_h2::RecvBody\">RecvBody</a>&gt;&gt;&gt;::<a class=\"type\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_service/trait.MakeService.html#associatedtype.Future\" title=\"type tower_util::make_service::MakeService::Future\">Future</a>: <a class=\"trait\" href=\"https://docs.rs/futures/0.1/futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as <a class=\"trait\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_service/trait.MakeService.html\" title=\"trait tower_util::make_service::MakeService\">MakeService</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://docs.rs/http/0.1.17/http/request/struct.Request.html\" title=\"struct http::request::Request\">Request</a>&lt;<a class=\"struct\" href=\"tower_h2/struct.RecvBody.html\" title=\"struct tower_h2::RecvBody\">RecvBody</a>&gt;&gt;&gt;::<a class=\"type\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_service/trait.MakeService.html#associatedtype.MakeError\" title=\"type tower_util::make_service::MakeService::MakeError\">MakeError</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as <a class=\"trait\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_service/trait.MakeService.html\" title=\"trait tower_util::make_service::MakeService\">MakeService</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://docs.rs/http/0.1.17/http/request/struct.Request.html\" title=\"struct http::request::Request\">Request</a>&lt;<a class=\"struct\" href=\"tower_h2/struct.RecvBody.html\" title=\"struct tower_h2::RecvBody\">RecvBody</a>&gt;&gt;&gt;::<a class=\"type\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_service/trait.MakeService.html#associatedtype.Service\" title=\"type tower_util::make_service::MakeService::Service\">Service</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>",synthetic:true,types:["tower_h2::server::Connection"]},{text:"impl&lt;T, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"tower_h2/server/struct.Background.html\" title=\"struct tower_h2::server::Background\">Background</a>&lt;T, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;B as <a class=\"trait\" href=\"tower_h2/trait.Body.html\" title=\"trait tower_h2::Body\">Body</a>&gt;::<a class=\"type\" href=\"tower_h2/trait.Body.html#associatedtype.Data\" title=\"type tower_h2::Body::Data\">Data</a>: <a class=\"trait\" href=\"https://docs.rs/bytes/0.4.12/bytes/buf/buf/trait.Buf.html\" title=\"trait bytes::buf::buf::Buf\">Buf</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>",synthetic:true,types:["tower_h2::server::Background"]},{text:"impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"tower_h2/server/enum.Error.html\" title=\"enum tower_h2::server::Error\">Error</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as <a class=\"trait\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_service/trait.MakeService.html\" title=\"trait tower_util::make_service::MakeService\">MakeService</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://docs.rs/http/0.1.17/http/request/struct.Request.html\" title=\"struct http::request::Request\">Request</a>&lt;<a class=\"struct\" href=\"tower_h2/struct.RecvBody.html\" title=\"struct tower_h2::RecvBody\">RecvBody</a>&gt;&gt;&gt;::<a class=\"type\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_service/trait.MakeService.html#associatedtype.Error\" title=\"type tower_util::make_service::MakeService::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as <a class=\"trait\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_service/trait.MakeService.html\" title=\"trait tower_util::make_service::MakeService\">MakeService</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://docs.rs/http/0.1.17/http/request/struct.Request.html\" title=\"struct http::request::Request\">Request</a>&lt;<a class=\"struct\" href=\"tower_h2/struct.RecvBody.html\" title=\"struct tower_h2::RecvBody\">RecvBody</a>&gt;&gt;&gt;::<a class=\"type\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_service/trait.MakeService.html#associatedtype.MakeError\" title=\"type tower_util::make_service::MakeService::MakeError\">MakeError</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>",synthetic:true,types:["tower_h2::server::Error"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
