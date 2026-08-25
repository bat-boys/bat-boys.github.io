let wasm;

self.onmessage = async ({ data }) => {
  try {
    wasm ??= await import("./pkg/lights_colors_out.js");
    await wasm.default();

    const progress = (processed, queueSize) => {
      self.postMessage({ type: "progress", processed, queueSize });
    };
    const moves = wasm.solve_wasm(data.puzzle, data.start, data.configuration, progress);
    self.postMessage({ type: "result", moves });
  } catch (error) {
    self.postMessage({ type: "error", message: error?.message ?? String(error) });
  }
};
