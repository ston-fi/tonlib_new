use crate::clients::lite_client::client::LiteClient;
use crate::types::tlb::block_tlb::block::block_id_ext::BlockIdExt;
use crate::types::tlb::block_tlb::block::Block;
use std::collections::HashSet;
use std::sync::Arc;

// use std::collections::HashSet;
// use std::sync::Arc;
// use std::sync::atomic::AtomicBool;
// use async_recursion::async_recursion;
// use crate::clients::lite::lite_client::LiteClient;
// use crate::clients::tonlibjson::tl_client::TLClient;
// use crate::errors::TonlibError;
// use crate::types::tlb::block_tlb::block::{Block, BlockIdExt};
//
#[derive(Debug, Clone)]
pub struct BlockStreamItem {
    pub blocks: Vec<(BlockIdExt, Block)>, // mc_block first, then it's shard_blocks
}

impl BlockStreamItem {
    pub fn mc_seqno(&self) -> u32 { todo!() }
}
//
pub struct BlockStream {
    inner: Arc<Inner>,
}

impl BlockStream {
    pub fn new(client: LiteClient, from_seqno: u32, to_seqno: Option<u32>) -> Self {
        todo!();
        // let inner = Arc::new(Inner {client})
        // BlockStream { client }
    }

    pub fn next(&self) -> Option<BlockStreamItem> {
        // let connection = self.client.get_connection().await?;
        // let result = connection.send("get_block", block_id).await;
        // match result {
        //     Ok(block) => Ok(block),
        //     Err(e) => Err(format!("Error getting block: {}", e)),
        // }
        todo!()
    }

    pub async fn get_block(&self, block_id: &str) -> Result<BlockStreamItem, String> {
        // let connection = self.client.get_connection().await?;
        // let result = connection.send("get_block", block_id).await;
        // match result {
        //     Ok(block) => Ok(block),
        //     Err(e) => Err(format!("Error getting block: {}", e)),
        // }
        todo!()
    }
}
//
struct Inner {
    client: LiteClient,
    to_mc_seqno: u32,
    prev_mc_seqno: u32,
    prev_last_shards: HashSet<BlockIdExt>, // the latest workchain shards for prev_mc_seqno
}
//
// impl Inner {
//
//     /// in case of error, can be called again safely
//     /// return Ok(None) if stream is over by "to_mc_seqno"
//     pub async fn next(&mut self) -> Result<Option<BlockStreamItem>, TonlibError> {
//         if self.prev_mc_seqno >= self.to_mc_seqno {
//             return Ok(None);
//         }
//         let seeking_mc_seqno = self.prev_mc_seqno + 1;
//         let mc_block_id = self.client.lookup_mc_block(seeking_mc_seqno).await?;
//         let mc_block = self.client.get_block(mc_block_id).await?;
//
//         let shard_ids = extract_shards_from_master_block(&mc_block_unpacked.block).await?;
//         log::trace!("Got mc_block: {mc_block:?} with shards: {shard_ids:?}");
//
//         let unseen_shards = self
//             .get_unseen_prev_blocks(seeking_mc_seqno, &shard_ids)
//             .await?
//             .into_iter()
//             .unique_by(|(id, _)| id.clone())
//             .map(|(_, block)| block)
//             .collect();
//
//         let result = Some(BlockStreamItem {
//             mc_block: (BlockIdExt {}, Block {}),
//             shard_blocks: unseen_shards,
//         });
//
//         // update it at the end to maintain exception-safety
//         self.prev_mc_seqno = seeking_mc_seqno;
//         self.prev_last_shards = shard_ids.into_iter().collect();
//
//         Ok(result)
//     }
//
//     #[async_recursion]
//     async fn get_unseen_prev_blocks(&self, mc_seqno: u32, shard_ids: &[BlockIdExt]) -> Result<Vec<Block>, TonlibError> {
//         let blocks_fut = shard_ids.iter().map(|block_id| async {
//             let block_id = block_id.clone(); // copy to scope
//
//             // blockchain feels bad about seq_no == 0:
//             // got unexpected response Error(Error { code: 651, message: String(not in db) })
//             // So technically block exists, but we can't get it
//             if self.prev_last_shards.contains(&block_id) || block_id.seqno == 0 {
//                 return Ok(vec![]);
//             }
//             // get cur block
//             let cur_block_data = self.client.get_block(block_id).await?;
//             // let cur_block = Block::from(cur_block_data); // todo
//             let unpacked =
//                 BlockUnpacked::new(cur_block, block_id.root_hash.inner().into(), block_id.file_hash.inner().into());
//
//             // recursively get previous block
//             let prev_ids = cur_block.info.read_prev_ids()?.into_iter().collect_vec();
//             let mut prev_blocks = self.get_unseen_prev_blocks(mc_seqno, &prev_ids).await?;
//             prev_blocks.push((block_id, cur_block));
//
//             Ok(prev_blocks)
//         });
//
//         // maybe worth check size before/after unique to confirm we don't make unnecessary lite calls
//         let blocks = try_join_all(blocks_fut).await?.into_iter().flatten().collect();
//         Ok(blocks)
//     }
// }
//
//
//
// fn extract_shards_from_master_block(mc_block: &Block) -> Result<Vec<BlockIdExt>, TonlibError> {
//     let extra = mc_block.read_extra()?;
//     let custom = extra.read_custom()?.unwrap(); // unwrap is safe for mc block
//     let shards = custom.shards();
//
//     let mut shard_block_ids = vec![];
//     shards.iterate_shards(|ident, shard| {
//         let root_hash = UInt256::from_slice(shard.root_hash.as_slice());
//         let file_hash = UInt256::from_slice(shard.file_hash.as_slice());
//         let shard_id = BlockIdExt::with_params(ident, shard.seq_no, root_hash, file_hash);
//         shard_block_ids.push(shard_id);
//         Ok(true)
//     })?;
//     Ok(shard_block_ids)
// }
