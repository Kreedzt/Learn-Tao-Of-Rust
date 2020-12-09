var mod;
var imports = {
  logit: () => {
    console.log('this was invoked by Rust, written js JS');
  },
  hello: (ptr, len)=> {
    var buf = new Uint8Array(
      // 得到 ArrayBuffer 对象以供操作
      mod.instance.exports.memory.buffer, ptr, len
    );
    var msg = new TextDecoder('utf8').decode(buf);
    alert(msg);
  }
}

fetch('output/small_hello.wasm')
  .then(res => res.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes, { env: imports }))
  .then(module => {
    mod = module;
    module.instance.exports.add_one(41);
  });
