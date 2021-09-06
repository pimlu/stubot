import { AiQuery } from './types';
import init, {WasmSearcher, WasmState} from './wasm';

const initted = init();

async function doSearch({fen, depth}: AiQuery) {
  await initted;
  const searcher = new WasmSearcher();
  const { score, mv } = searcher.search(new WasmState(fen), depth);

  const { nodes } = searcher;
  postMessage({ score, mv, nodes});
}
onmessage = (e) => {
  doSearch(e.data as AiQuery);
};
