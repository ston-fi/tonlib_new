// use rand::prelude::IndexedRandom;
// use crate::clients::tonlib::tl::request::TonlibRequest;
// use crate::clients::tonlib::tl::response::TonlibResponse;
// use crate::clients::tonlib::traits::TonlibExecutor;
// use crate::errors::TonLibError;
//
// pub struct TonlibClient<T> {
//     inner: Inner<T>
// }
//
// impl<T: TonlibExecutor> TonlibClient<T> {
//     pub fn get_executor(&self) -> &impl TonlibExecutor {
//         self.inner.executors_pool.choose(&mut rand::rng()).unwrap() // fine unless pool is empty
//     }
//
//     pub async fn execute_on(&self, req: &TonlibRequest, executor: &impl TonlibExecutor) -> Result<TonlibResponse, TonLibError> {
//
//     }
// }
//
// impl<T: TonlibExecutor> TonlibExecutor for TonlibClient<T> {
//     async fn execute(&self, req: &TonlibRequest) -> Result<TonlibResponse, TonLibError> {
//         self.get_executor().execute(&req).await
//     }
// }
//
// struct Inner<T> {
//     executors_pool: Vec<T>,
//     _retry_strategy: u32, // todo
// }
//
// impl<T: TonlibExecutor> TonlibClient<T> {
//
// }
