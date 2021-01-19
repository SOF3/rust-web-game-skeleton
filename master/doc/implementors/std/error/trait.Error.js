(function() {var implementors = {};
implementors["bincode"] = [{"text":"impl Error for ErrorKind","synthetic":false,"types":[]}];
implementors["common"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["digest"] = [{"text":"impl Error for InvalidOutputSize","synthetic":false,"types":[]}];
implementors["futures_channel"] = [{"text":"impl Error for SendError","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Any&gt; Error for TrySendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Error for TryRecvError","synthetic":false,"types":[]},{"text":"impl Error for Canceled","synthetic":false,"types":[]}];
implementors["futures_executor"] = [{"text":"impl Error for EnterError","synthetic":false,"types":[]}];
implementors["futures_task"] = [{"text":"impl Error for SpawnError","synthetic":false,"types":[]}];
implementors["futures_util"] = [{"text":"impl Error for Aborted","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Any, Item&gt; Error for ReuniteError&lt;T, Item&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Any&gt; Error for ReuniteError&lt;T&gt;","synthetic":false,"types":[]}];
implementors["getrandom"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["gloo_file"] = [{"text":"impl Error for FileReadError","synthetic":false,"types":[]}];
implementors["http"] = [{"text":"impl Error for InvalidHeaderName","synthetic":false,"types":[]},{"text":"impl Error for InvalidHeaderValue","synthetic":false,"types":[]},{"text":"impl Error for ToStrError","synthetic":false,"types":[]},{"text":"impl Error for InvalidMethod","synthetic":false,"types":[]},{"text":"impl Error for InvalidStatusCode","synthetic":false,"types":[]},{"text":"impl Error for InvalidUri","synthetic":false,"types":[]},{"text":"impl Error for InvalidUriParts","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["log"] = [{"text":"impl Error for SetLoggerError","synthetic":false,"types":[]},{"text":"impl Error for ParseLevelError","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl Error for LexError","synthetic":false,"types":[]}];
implementors["rand"] = [{"text":"impl Error for BernoulliError","synthetic":false,"types":[]},{"text":"impl Error for WeightedError","synthetic":false,"types":[]},{"text":"impl Error for ReadError","synthetic":false,"types":[]}];
implementors["rand_core"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["serde"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["serde_json"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["yew"] = [{"text":"impl Error for FormatError","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()