var e,t=Object.defineProperty,n=Object.getOwnPropertySymbols,r=Object.prototype.hasOwnProperty,a=Object.prototype.propertyIsEnumerable,s=(e,n,r)=>n in e?t(e,n,{enumerable:!0,configurable:!0,writable:!0,value:r}):e[n]=r,o=(e,t)=>{for(var o in t||(t={}))r.call(t,o)&&s(e,o,t[o]);if(n)for(var o of n(t))a.call(t,o)&&s(e,o,t[o]);return e},i=(e,t)=>{var s={};for(var o in e)r.call(e,o)&&t.indexOf(o)<0&&(s[o]=e[o]);if(null!=e&&n)for(var o of n(e))t.indexOf(o)<0&&a.call(e,o)&&(s[o]=e[o]);return s};import{R as c,u as l,F as u,a as d,f as m,b as p,c as f,d as _,e as w,g as b,r as g,h as y,D as v,i as h}from"./vendor.889ffd78.js";let E;!function(){const e=document.createElement("link").relList;if(!(e&&e.supports&&e.supports("modulepreload"))){for(const e of document.querySelectorAll('link[rel="modulepreload"]'))t(e);new MutationObserver((e=>{for(const n of e)if("childList"===n.type)for(const e of n.addedNodes)"LINK"===e.tagName&&"modulepreload"===e.rel&&t(e)})).observe(document,{childList:!0,subtree:!0})}function t(e){if(e.ep)return;e.ep=!0;const t=function(e){const t={};return e.integrity&&(t.integrity=e.integrity),e.referrerpolicy&&(t.referrerPolicy=e.referrerpolicy),"use-credentials"===e.crossorigin?t.credentials="include":"anonymous"===e.crossorigin?t.credentials="omit":t.credentials="same-origin",t}(e);fetch(e.href,t)}}();let k=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});k.decode();let x=null;function S(){return null!==x&&x.buffer===E.memory.buffer||(x=new Uint8Array(E.memory.buffer)),x}function M(e,t){return k.decode(S().subarray(e,e+t))}let N=0,R=new TextEncoder("utf-8");const W="function"==typeof R.encodeInto?function(e,t){return R.encodeInto(e,t)}:function(e,t){const n=R.encode(e);return t.set(n),{read:e.length,written:n.length}};function A(e,t,n){if(void 0===n){const n=R.encode(e),r=t(n.length);return S().subarray(r,r+n.length).set(n),N=n.length,r}let r=e.length,a=t(r);const s=S();let o=0;for(;o<r;o++){const t=e.charCodeAt(o);if(t>127)break;s[a+o]=t}if(o!==r){0!==o&&(e=e.slice(o)),a=n(a,r,r=o+3*e.length);const t=S().subarray(a+o,a+r);o+=W(e,t).written}return N=o,a}let C=null;function O(){return null!==C&&C.buffer===E.memory.buffer||(C=new Int32Array(E.memory.buffer)),C}class P{static __wrap(e){const t=Object.create(P.prototype);return t.ptr=e,t}__destroy_into_raw(){const e=this.ptr;return this.ptr=0,e}free(){const e=this.__destroy_into_raw();E.__wbg_wasmpos_free(e)}constructor(e,t){var n=E.wasmpos_new(e,t);return P.__wrap(n)}toString(){try{const n=E.__wbindgen_add_to_stack_pointer(-16);E.wasmpos_toString(n,this.ptr);var e=O()[n/4+0],t=O()[n/4+1];return M(e,t)}finally{E.__wbindgen_add_to_stack_pointer(16),E.__wbindgen_free(e,t)}}}class I{static __wrap(e){const t=Object.create(I.prototype);return t.ptr=e,t}__destroy_into_raw(){const e=this.ptr;return this.ptr=0,e}free(){const e=this.__destroy_into_raw();E.__wbg_wasmstate_free(e)}constructor(e){var t=null==e?0:A(e,E.__wbindgen_malloc,E.__wbindgen_realloc),n=N,r=E.wasmstate_new(t,n);return I.__wrap(r)}toString(){try{const n=E.__wbindgen_add_to_stack_pointer(-16);E.wasmstate_toString(n,this.ptr);var e=O()[n/4+0],t=O()[n/4+1];return M(e,t)}finally{E.__wbindgen_add_to_stack_pointer(16),E.__wbindgen_free(e,t)}}moveGen(){try{const n=E.__wbindgen_add_to_stack_pointer(-16);E.wasmstate_moveGen(n,this.ptr);var e=O()[n/4+0],t=O()[n/4+1];return M(e,t)}finally{E.__wbindgen_add_to_stack_pointer(16),E.__wbindgen_free(e,t)}}makeMove(e){var t=A(e,E.__wbindgen_malloc,E.__wbindgen_realloc),n=N;E.wasmstate_makeMove(this.ptr,t,n)}score(){return E.wasmstate_score(this.ptr)}boardString(){try{const n=E.__wbindgen_add_to_stack_pointer(-16);E.wasmstate_boardString(n,this.ptr);var e=O()[n/4+0],t=O()[n/4+1];return M(e,t)}finally{E.__wbindgen_add_to_stack_pointer(16),E.__wbindgen_free(e,t)}}isWhite(){return 0!==E.wasmstate_isWhite(this.ptr)}}async function q(e){void 0===e&&(e=new URL("./assets/stubot_wasm_bg.f122d092.wasm",window.location));const t={wbg:{}};t.wbg.__wbindgen_throw=function(e,t){throw new Error(M(e,t))},("string"==typeof e||"function"==typeof Request&&e instanceof Request||"function"==typeof URL&&e instanceof URL)&&(e=fetch(e));const{instance:n,module:r}=await async function(e,t){if("function"==typeof Response&&e instanceof Response){if("function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(e,t)}catch(n){if("application/wasm"==e.headers.get("Content-Type"))throw n;console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",n)}const r=await e.arrayBuffer();return await WebAssembly.instantiate(r,t)}{const n=await WebAssembly.instantiate(e,t);return n instanceof WebAssembly.Instance?{instance:n,module:e}:n}}(await e,t);return E=n.exports,q.__wbindgen_wasm_module=r,E.__wbindgen_start(),E}let L,$,j=!1;const D=new Promise(((e,t)=>{L=e,$=t}));function U(){return async function(){let e="./assets/stubot_wasm_bg.f122d092.wasm";void 0===self.document&&(e=e.replace("./assets",".")),await q(new URL(e,`${self.location}`)),j=!0}().then(L).catch($),D}const B=c.memo(c.forwardRef((function({dark:e,bg:t,desc:n,children:r},a){return c.createElement("div",{ref:a,role:"gridcell",className:"sq css-sq "+(e?"db":"lb")},c.createElement("div",{className:`sq-inner fit-abs ${t||""}`,"aria-label":n},r))})));function T(e){var t=e,{pos:n,bg:r}=t,a=i(t,["pos","bg"]);const{setNodeRef:s}=l({id:n});return c.createElement(B,o({ref:s},o({pos:n,bg:r},a)))}const F={P:m,N:p,B:f,R:_,Q:w,K:b},G=c.memo(c.forwardRef((function(e,t){var n=e,{pc:r}=n,a=i(n,["pc"]);const s=r.toUpperCase(),l=r===s,d=F[s];return c.createElement("div",o({className:"pc-frame center fit-pct",ref:t},a),c.createElement("div",{className:"pc"},c.createElement(u,{icon:d,className:"fit-pct "+(l?"w":"b")})))})));function K({pos:e,pc:t}){const{attributes:n,listeners:r,setNodeRef:a,transform:s,isDragging:i}=d({id:e}),l=s?{transform:`translate3d(${s.x}px, ${s.y}px, 0)`,zIndex:i?2:void 0}:void 0;return c.createElement(G,o(o({ref:a,pc:t,id:`pc-${e}`,style:l},r),n))}function z(){return new Worker("./assets/Worker.6a50cdf0.js",{type:"module"})}const Q=null!=(e=window.requestIdleCallback)?e:queueMicrotask,H=([e,t,n,r])=>[e+t,n+r];let J;Q((()=>{J||(J=new z)}));function V({state:e,mkMove:t,canMove:n,flipped:r}){const{grid:a,mvMap:s}=g.exports.useMemo((()=>({grid:e.st.boardString().split("\n").reverse().map((e=>e.split(" "))),mvMap:e.st.moveGen().split(" ").map(H).reduce(((e,[t,n])=>(e.get(t)||e.set(t,new Set),e.get(t).add(n),e)),new Map)})),[e]),i=a.length,l=a[0].length,[u,d]=g.exports.useState(),m=e=>{var t;return!!u&&!!e&&(null==(t=s.get(u))?void 0:t.has(e))};function p(){d(void 0)}y({onDragStart(e){d(e.active.id)},onDragEnd:function(e){var r;const a=null==(r=e.over)?void 0:r.id;n&&m(a)&&t(u,a),p()},onDragCancel:p});const f=[...Array(l)].map(((e,t)=>t)),_=[...Array(i)].map(((e,t)=>t));return r||f.reverse(),r&&_.reverse(),c.createElement("div",{className:"board-wrap css-sq"},c.createElement("div",{className:"board",role:"grid",style:{gridTemplateColumns:`repeat(${i}, 1fr)`,gridTemplateRows:`repeat(${l}, 1fr)`}},f.flatMap((e=>c.createElement("div",{key:e,role:"row",className:"d-contents"},_.map((t=>{const n=a[e][t],r=`${new P(e,t)}`,i=(e+t)%2==0,l=s.has(r)?K:G,d="."!==n,p=d&&c.createElement(l,o({},{pos:r,pc:n})),f=u===r,_=m(r),w=f?"cover":_?d?"frame":"circ":void 0,b=r;return c.createElement(T,o({key:r},{desc:b,pos:r,dark:i,bg:w}),p)})))))))}class X{constructor(e){var t,n;s(this,"symbol"!=typeof(t="st")?t+"":t,n),this.st=e}mut(e){return e(this.st),new X(this.st)}}const Y=c.memo((function({bot:e}){const[t,n]=g.exports.useState((()=>new X(new I))),r=g.exports.useMemo((()=>t.st.isWhite()),[t]),a=e?e.isWhite:!r,s=g.exports.useCallback(((e,t)=>n((n=>n.mut((n=>n.makeMove(e+t)))))),[n]),i=!e||e.isWhite!==r;return g.exports.useEffect((()=>{if(i||!e)return;let n=!1;const{promise:r,cancel:a}=(o={fen:`${t.st}`,depth:e.depth},J||(J=new z),{promise:new Promise(((e,t)=>{J.onmessage=t=>{e(t.data)},J.onerror=t,J.postMessage(o)})),cancel(){null==J||J.onerror(new Error("cancelled")),null==J||J.terminate(),J=void 0}});var o;return r.then((({mv:e})=>{n=!0,e&&s(...H(e))})),()=>{n||a()}}),[r,e,i]),c.createElement(v,null,c.createElement(V,o({},{state:t,mkMove:s,canMove:i,flipped:a})))}));function Z(e){const[t,n]=g.exports.useState(j);return g.exports.useEffect((()=>{t||D.then((()=>n(!0)))}),[t]),t?c.createElement(Y,o({},e)):c.createElement(c.Fragment,null,"Loading...")}function ee({setPhase:e}){const[t,n]=g.exports.useState("engine"),[r,a]=g.exports.useState(4),[s,i]=g.exports.useState("random");const l=(e,t,n=!1)=>r=>({value:r,checked:r===e,disabled:n,onChange(e){t(e.target.value)}}),u=l(t,n),d=l(s,i,"engine"!==t);return c.createElement("div",{className:"intro"},c.createElement("h1",null,"Rust Chess"),c.createElement("div",{className:"flex-col"},c.createElement("div",null,c.createElement("label",null,c.createElement("input",o({type:"radio",name:"opponent"},u("engine"))),"vs. Engine"," "),c.createElement("label",{onClick:()=>n("engine")},"level ",r,c.createElement("input",{type:"range",disabled:"engine"!==t,min:1,max:7,value:r,onChange:e=>a(+e.target.value)})),c.createElement("div",{className:"indent",onClick:()=>n("engine")},"Play as:"," ",c.createElement("label",null,c.createElement("input",o({type:"radio",name:"color"},d("white")))," White"),c.createElement("label",null,c.createElement("input",o({type:"radio",name:"color"},d("black")))," Black"),c.createElement("label",null,c.createElement("input",o({type:"radio",name:"color"},d("random")))," Random"))),c.createElement("label",null,c.createElement("input",o({type:"radio",name:"opponent"},u("friend"))),"vs. Friend (local)"),c.createElement("div",null,c.createElement("button",{className:"start-btn",onClick:function(){const n="random"===s?Math.random()<.5:"white"===s;e({cur:"game",bot:"friend"===t?void 0:{depth:r+1,isWhite:!n}})}},"Start"))))}function te(){const[e,t]=g.exports.useState({cur:"intro"});switch(e.cur){case"intro":return c.createElement(ee,o({},{setPhase:t}));case"game":{const{bot:n}=e;return c.createElement(Z,o({},{setPhase:t,bot:n}))}default:throw new Error("bad phase")}}Q((()=>{U()}),{timeout:250}),h.exports.render(c.createElement(te,null),document.getElementById("app"));
