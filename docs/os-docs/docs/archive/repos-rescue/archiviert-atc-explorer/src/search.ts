// Search functionality
import { BlockBrowser, BlockInfo } from './blocks';
import { TransactionBrowser, TxInfo } from './transactions';

export class Search {
  constructor(private blocks: BlockBrowser, private txs: TransactionBrowser) {}

  query(term: string): { blocks: BlockInfo[]; txs: TxInfo[] } {
    const height = parseInt(term);
    const block = !isNaN(height) ? this.blocks.getBlock(height) : null;
    const tx = this.txs.getTx(term);

    return {
      blocks: block ? [block] : [],
      txs: tx ? [tx] : [],
    };
  }
}
