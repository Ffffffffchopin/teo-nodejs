use napi::{JsFunction, Result};
use napi::threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction};
use teo::prelude::{Arguments, Namespace as TeoNamespace, Model as TeoModel, model::Field as TeoField, model::Property as TeoProperty, model::Relation as TeoRelation, object::Object as TeoObject, Arguments as TeoArgs, pipeline, model, transaction, request, response::Response as TeoResponse};
use crate::dynamic::{js_ctx_object_from_teo_transaction_ctx, js_model_object_from_teo_model_object};
use crate::model::model::Model;
use crate::model::relation::relation::Relation;
use crate::model::field::field::Field;
use crate::model::property::property::Property;
use crate::object::promise::TeoObjectOrPromise;
use crate::object::teo_object_to_js_any;
use crate::object::arguments::teo_args_to_js_args;
use crate::object::value::teo_value_to_js_any;
use crate::request::Request;
use crate::response::response_or_promise::ResponseOrPromise;

#[napi(js_name = "Namespace")]
pub struct Namespace {
    pub(crate) teo_namespace: &'static mut TeoNamespace,
}

#[napi]
impl Namespace {

    #[napi(js_name = "isMain")]
    pub fn is_main(&self) -> bool {
        self.teo_namespace.is_main()
    }

    #[napi(js_name = "isStd")]
    pub fn is_std(&self) -> bool {
        self.teo_namespace.is_std()
    }

    #[napi]
    pub fn path(&self) -> Vec<&str> {
        self.teo_namespace.path()
    }

    #[napi]
    pub fn namespace(&'static mut self, name: String) -> Option<Namespace> {
        self.teo_namespace.namespace_mut(name.as_str()).map(|n| Namespace { teo_namespace: n })
    }

    #[napi]
    pub fn namespace_or_create(&'static mut self, name: String) -> Namespace {
        Namespace{ teo_namespace: self.teo_namespace.namespace_mut_or_create(name.as_str()) }
    }

    #[napi]
    pub fn namespace_at_path(&'static mut self, path: Vec<&str>) -> Option<Namespace> {
        self.teo_namespace.namespace_mut_at_path(&path).map(|n| Namespace { teo_namespace: n })
    }

    #[napi]
    pub fn namespace_or_create_at_path(&'static mut self, path: Vec<&str>) -> Namespace {
        Namespace{ teo_namespace: self.teo_namespace.namespace_mut_or_create_at_path(&path) }
    }

    #[napi(js_name = "defineModelDecorator", ts_args_type = "name: string, body: (model: Model) => void")]
    pub fn define_model_decorator(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(teo::prelude::Arguments, &mut TeoModel), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(Arguments, &mut TeoModel)>| {
            let arguments = teo_args_to_js_args(&ctx.value.0, &ctx.env)?;
            let js_model = Model { teo_model: ctx.value.1 };
            Ok(vec![arguments, js_model.into_instance(ctx.env)?.as_object(ctx.env)])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_model_decorator(name.as_str(), |arguments, model| {
            let static_model: &'static mut TeoModel = unsafe { &mut *(model as * mut TeoModel) };
            let _ = tsfn_cloned.call((arguments, static_model), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        });
        Ok(())
    }

    #[napi(js_name = "defineModelFieldDecorator", ts_args_type = "name: string, body: (field: Field) => void")]
    pub fn define_model_field_decorator(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(teo::prelude::Arguments, &mut TeoField), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(Arguments, &mut TeoField)>| {
            let arguments = teo_args_to_js_args(&ctx.value.0, &ctx.env)?;
            let js_model = Field { teo_field: ctx.value.1 };
            Ok(vec![arguments, js_model.into_instance(ctx.env)?.as_object(ctx.env)])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_model_field_decorator(name.as_str(), |arguments, model| {
            let static_model: &'static mut TeoField = unsafe { &mut *(model as * mut TeoField) };
            let _ = tsfn_cloned.call((arguments, static_model), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        });
        Ok(())
    }

    #[napi(js_name = "defineModelRelationDecorator", ts_args_type = "name: string, body: (relation: Relation) => void")]
    pub fn define_model_relation_decorator(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(teo::prelude::Arguments, &mut TeoRelation), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(Arguments, &mut TeoRelation)>| {
            let arguments = teo_args_to_js_args(&ctx.value.0, &ctx.env)?;
            let js_model = Relation { teo_relation: ctx.value.1 };
            Ok(vec![arguments, js_model.into_instance(ctx.env)?.as_object(ctx.env)])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_model_relation_decorator(name.as_str(), |arguments, model| {
            let static_model: &'static mut TeoRelation = unsafe { &mut *(model as * mut TeoRelation) };
            let _ = tsfn_cloned.call((arguments, static_model), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        });
        Ok(())
    }

    #[napi(js_name = "defineModelPropertyDecorator", ts_args_type = "name: string, body: (property: Property) => void")]
    pub fn define_model_property_decorator(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(teo::prelude::Arguments, &mut TeoProperty), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(Arguments, &mut TeoProperty)>| {
            let arguments = teo_args_to_js_args(&ctx.value.0, &ctx.env)?;
            let js_model = Property { teo_property: ctx.value.1 };
            Ok(vec![arguments, js_model.into_instance(ctx.env)?.as_object(ctx.env)])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_model_property_decorator(name.as_str(), |arguments, model| {
            let static_model: &'static mut TeoProperty = unsafe { &mut *(model as * mut TeoProperty) };
            let _ = tsfn_cloned.call((arguments, static_model), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            Ok(())
        });
        Ok(())
    }

    #[napi(js_name = "definePipelineItem", ts_args_type = "name: string, body: (value: any, args?: {[key: string]: any}, object?: any, ctx?: any) => any | Promise<any>")]
    pub fn define_pipeline_item(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(TeoObject, TeoArgs, model::Object, transaction::Ctx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoObject, TeoArgs, model::Object, transaction::Ctx)>| {
            let js_value = teo_object_to_js_any(&ctx.value.0, &ctx.env)?;
            let js_args = teo_args_to_js_args(&ctx.value.1, &ctx.env)?;
            let js_object = js_model_object_from_teo_model_object(ctx.env, ctx.value.2.clone())?;
            let js_ctx = js_ctx_object_from_teo_transaction_ctx(ctx.env, ctx.value.3.clone(), "")?;
            Ok(vec![js_value, js_args.into_unknown(), js_object.into_unknown(), js_ctx.into_unknown()])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_pipeline_item(name.as_str(), move |args: TeoArgs, ctx: pipeline::Ctx| async move {
            let object = ctx.value().clone();
            let model_object = ctx.object().clone();
            let transaction_ctx = ctx.transaction_ctx().clone();
            let result: TeoObjectOrPromise = tsfn_cloned.call_async((object, args, model_object, transaction_ctx)).await.unwrap();
            Ok(result.to_teo_object().await.unwrap())
        });
        Ok(())
    }

    #[napi(js_name = "defineHandler")]
    pub fn define_handler(&mut self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<request::Ctx, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<request::Ctx>| {
            let teo_request = ctx.value.request().clone();
            let request = Request::new(teo_request);
            let request_instance = request.into_instance(ctx.env)?;
            let request_unknown = request_instance.as_object(ctx.env).into_unknown();
            let body = teo_value_to_js_any(&ctx.value.body(), &ctx.env)?;
            let js_ctx = js_ctx_object_from_teo_transaction_ctx(ctx.env, ctx.value.transaction_ctx(), "")?.into_unknown();
            Ok(vec![body, js_ctx, request_unknown])
        })?;
        let tsfn_cloned = &*Box::leak(Box::new(tsfn));
        self.teo_namespace.define_handler(name.as_str(), move |ctx: request::Ctx| async move {
            let response_unknown: ResponseOrPromise = tsfn_cloned.call_async(ctx).await.unwrap();
            Ok::<TeoResponse, teo::prelude::path::Error>(response_unknown.to_teo_response().await.unwrap())
        });
        Ok(())
    }
}

