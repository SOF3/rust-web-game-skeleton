(function() {var implementors = {};
implementors["byteorder"] = [{"text":"impl PartialOrd&lt;BigEndian&gt; for BigEndian","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;LittleEndian&gt; for LittleEndian","synthetic":false,"types":[]}];
implementors["bytes"] = [{"text":"impl PartialOrd&lt;Bytes&gt; for Bytes","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;[u8]&gt; for Bytes","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Bytes&gt; for [u8]","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;str&gt; for Bytes","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Vec&lt;u8, Global&gt;&gt; for Bytes","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;String&gt; for Bytes","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Bytes&gt; for &amp;[u8]","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;?Sized&gt; PartialOrd&lt;&amp;'a T&gt; for Bytes <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Bytes: PartialOrd&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;BytesMut&gt; for BytesMut","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;[u8]&gt; for BytesMut","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;BytesMut&gt; for [u8]","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;str&gt; for BytesMut","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Vec&lt;u8, Global&gt;&gt; for BytesMut","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;String&gt; for BytesMut","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;?Sized&gt; PartialOrd&lt;&amp;'a T&gt; for BytesMut <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;BytesMut: PartialOrd&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;BytesMut&gt; for &amp;[u8]","synthetic":false,"types":[]}];
implementors["futures_util"] = [{"text":"impl&lt;T:&nbsp;PartialOrd&gt; PartialOrd&lt;AllowStdIo&lt;T&gt;&gt; for AllowStdIo&lt;T&gt;","synthetic":false,"types":[]}];
implementors["generic_array"] = [{"text":"impl&lt;T:&nbsp;PartialOrd, N&gt; PartialOrd&lt;GenericArray&lt;T, N&gt;&gt; for GenericArray&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ArrayLength&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["http"] = [{"text":"impl PartialOrd&lt;HeaderValue&gt; for HeaderValue","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;str&gt; for HeaderValue","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;[u8]&gt; for HeaderValue","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;HeaderValue&gt; for str","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;String&gt; for HeaderValue","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; PartialOrd&lt;HeaderValue&gt; for &amp;'a HeaderValue","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;?Sized&gt; PartialOrd&lt;&amp;'a T&gt; for HeaderValue <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;HeaderValue: PartialOrd&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; PartialOrd&lt;HeaderValue&gt; for &amp;'a str","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;StatusCode&gt; for StatusCode","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Authority&gt; for Authority","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;str&gt; for Authority","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Authority&gt; for str","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; PartialOrd&lt;Authority&gt; for &amp;'a str","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; PartialOrd&lt;&amp;'a str&gt; for Authority","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;String&gt; for Authority","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;PathAndQuery&gt; for PathAndQuery","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;str&gt; for PathAndQuery","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;PathAndQuery&gt; for str","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; PartialOrd&lt;&amp;'a str&gt; for PathAndQuery","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; PartialOrd&lt;PathAndQuery&gt; for &amp;'a str","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;String&gt; for PathAndQuery","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Version&gt; for Version","synthetic":false,"types":[]}];
implementors["log"] = [{"text":"impl PartialOrd&lt;Level&gt; for Level","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;LevelFilter&gt; for Level","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;LevelFilter&gt; for LevelFilter","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Level&gt; for LevelFilter","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; PartialOrd&lt;Metadata&lt;'a&gt;&gt; for Metadata&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; PartialOrd&lt;MetadataBuilder&lt;'a&gt;&gt; for MetadataBuilder&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl PartialOrd&lt;Ident&gt; for Ident","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl PartialOrd&lt;Lifetime&gt; for Lifetime","synthetic":false,"types":[]}];
implementors["typenum"] = [{"text":"impl PartialOrd&lt;B0&gt; for B0","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;B1&gt; for B1","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;PartialOrd + Unsigned + NonZero&gt; PartialOrd&lt;PInt&lt;U&gt;&gt; for PInt&lt;U&gt;","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;PartialOrd + Unsigned + NonZero&gt; PartialOrd&lt;NInt&lt;U&gt;&gt; for NInt&lt;U&gt;","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Z0&gt; for Z0","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;UTerm&gt; for UTerm","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;PartialOrd, B:&nbsp;PartialOrd&gt; PartialOrd&lt;UInt&lt;U, B&gt;&gt; for UInt&lt;U, B&gt;","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;ATerm&gt; for ATerm","synthetic":false,"types":[]},{"text":"impl&lt;V:&nbsp;PartialOrd, A:&nbsp;PartialOrd&gt; PartialOrd&lt;TArr&lt;V, A&gt;&gt; for TArr&lt;V, A&gt;","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Greater&gt; for Greater","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Less&gt; for Less","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Equal&gt; for Equal","synthetic":false,"types":[]}];
implementors["yew"] = [{"text":"impl PartialOrd&lt;Key&gt; for Key","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()