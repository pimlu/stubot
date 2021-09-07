import { AiQuery, AiResponse, JsPos } from './types';
import Worker from './Worker?worker';


export const splitMv = ([ax, ay, bx, by]: string): [JsPos, JsPos] => [ax+ay, bx+by];

let worker: Worker | undefined;
requestIdleCallback(() => {
  if (!worker) worker = new Worker();
});
export function negamax(q: AiQuery) {
  if (!worker) worker = new Worker();
  const promise = new Promise<AiResponse>((res, rej) => {
    worker!.onmessage = (e) => {
      res (e.data as AiResponse);
    };
    worker!.onerror = rej;
    worker!.postMessage(q);
  });
  return {
    promise,
    cancel() {
      worker?.onerror!(new Error('cancelled') as any);
      worker?.terminate();
      worker = undefined;
    }
  };
}