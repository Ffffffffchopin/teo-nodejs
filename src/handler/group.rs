use napi::{Result, JsFunction, threadsafe_function::{ThreadSafeCallContext, ErrorStrategy, ThreadsafeFunction}};
use teo::prelude::{handler::Group as TeoHandlerGroup, request, Response as TeoResponse};

use crate::{dynamic::js_ctx_object_from_teo_transaction_ctx, object::value::teo_value_to_js_any, request::{Request, RequestCtx}, response::response_or_promise::ResponseOrPromise};

#[napi(js_name = "HandlerGroup")]
pub struct HandlerGroup {
    pub(crate) teo_handler_group: &'static mut TeoHandlerGroup,
}

#[napi]
impl HandlerGroup {

    #[napi(js_name = "defineHandler", ts_args_type = "name: string, callback: (ctx: RequestCtx) => Response | Promise<Response>")]
    pub fn define_handler(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<request::Ctx, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<request::Ctx>| {
            let request_ctx = RequestCtx::new(ctx.value.clone());
            let request_ctx_instance = request_ctx.into_instance(ctx.env)?;
            let request_ctx_unknown = request_ctx_instance.as_object(ctx.env).into_unknown();
            Ok(vec![request_ctx_unknown])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_handler_group.define_handler(name.as_str(), move |ctx: request::Ctx| async move {
            let response_unknown: ResponseOrPromise = tsfn_cloned.call_async(ctx).await.unwrap();
            Ok::<TeoResponse, teo::prelude::path::Error>(response_unknown.to_teo_response().await.unwrap())
        });
        Ok(())
    }
}