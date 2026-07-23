const { performance } = require("node:perf_hooks");
const path = require("node:path");

const modulePath = process.env.DOCX_WASM_MODULE
  ? path.resolve(process.env.DOCX_WASM_MODULE)
  : path.resolve(__dirname, "../dist/node");
const wasmPath = process.env.DOCX_WASM_BINARY
  ? path.resolve(process.env.DOCX_WASM_BINARY)
  : path.join(modulePath, "pkg/docx_wasm_bg.wasm");
const wasm = require(modulePath);

const PARAGRAPH_COUNT = Number(process.env.PARAGRAPH_COUNT ?? 2_000);
const WARMUP_ITERATIONS = Number(process.env.WARMUP_ITERATIONS ?? 3);
const MEASURED_ITERATIONS = Number(process.env.MEASURED_ITERATIONS ?? 10);

const template = new wasm.Docx();
for (let index = 0; index < PARAGRAPH_COUNT; index += 1) {
  template.addParagraph(
    new wasm.Paragraph().addRun(
      new wasm.Run().addText(
        `paragraph ${index}: lorem ipsum dolor sit amet, consectetur adipiscing elit`
      )
    )
  );
}

const samples = [];
let outputBytes = 0;
for (
  let iteration = 0;
  iteration < WARMUP_ITERATIONS + MEASURED_ITERATIONS;
  iteration += 1
) {
  const start = performance.now();
  const output = template.build();
  const elapsed = performance.now() - start;
  outputBytes = output.byteLength;
  if (iteration >= WARMUP_ITERATIONS) {
    samples.push(elapsed);
  }
}

samples.sort((left, right) => left - right);
const median = samples[Math.floor(samples.length / 2)];
const mean = samples.reduce((sum, sample) => sum + sample, 0) / samples.length;

console.log(
  JSON.stringify({
    modulePath,
    paragraphCount: PARAGRAPH_COUNT,
    iterations: MEASURED_ITERATIONS,
    outputBytes,
    wasmBytes: require("node:fs").statSync(wasmPath).size,
    medianMilliseconds: median,
    meanMilliseconds: mean,
    minMilliseconds: samples[0],
    maxMilliseconds: samples[samples.length - 1],
  })
);
