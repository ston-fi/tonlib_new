#[cfg(test)]
mod tests {
    use crate::tlb::block::BLOCK_TLB;

    extern crate colog;

    #[test]
    fn test_tlb_parser() -> anyhow::Result<()> {
        colog::init();
        let tlb_plain = BLOCK_TLB.to_string();
        let mut tlb_split_line: Vec<String> = tlb_plain.split(";").map(|x| x.to_string()).collect();
        #[allow(clippy::needless_range_loop)]
        for i in 0..tlb_split_line.len() {
            let cur_line = tlb_split_line[i].to_string();
            let new_line = cur_line.replace("\n", "");
            tlb_split_line[i] = new_line;
        }

        let mut line_num = 1;
        for line in &tlb_split_line {
            log::info!("[{line_num}] {line}",);
            line_num += 1;
            if line_num >= 100 {
                break;
            }
        }
        Ok(())
    }
}
