(function() {var implementors = {};
implementors["tower_h2"] = [{text:"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"tower_h2/client/enum.ConnectError.html\" title=\"enum tower_h2::client::ConnectError\">ConnectError</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a>,&nbsp;</span>",synthetic:false,types:["tower_h2::client::connect::ConnectError"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"tower_h2/client/struct.Error.html\" title=\"struct tower_h2::client::Error\">Error</a>",synthetic:false,types:["tower_h2::client::connection::Error"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"tower_h2/client/enum.HandshakeError.html\" title=\"enum tower_h2::client::HandshakeError\">HandshakeError</a>",synthetic:false,types:["tower_h2::client::connection::HandshakeError"]},{text:"impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"tower_h2/server/enum.Error.html\" title=\"enum tower_h2::server::Error\">Error</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_service/trait.MakeService.html\" title=\"trait tower_util::make_service::MakeService\">MakeService</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://docs.rs/http/0.1.17/http/request/struct.Request.html\" title=\"struct http::request::Request\">Request</a>&lt;<a class=\"struct\" href=\"tower_h2/struct.RecvBody.html\" title=\"struct tower_h2::RecvBody\">RecvBody</a>&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_service/trait.MakeService.html#associatedtype.MakeError\" title=\"type tower_util::make_service::MakeService::MakeError\">MakeError</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"https://docs.rs/tower-util/0.1.0/tower_util/make_service/trait.MakeService.html#associatedtype.Error\" title=\"type tower_util::make_service::MakeService::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a>,&nbsp;</span>",synthetic:false,types:["tower_h2::server::Error"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()