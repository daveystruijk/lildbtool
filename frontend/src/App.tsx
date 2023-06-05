import type { Component } from "solid-js";
import Table from "./Table";

import type { TableModel } from "../../backend/bindings/TableModel";

const App: Component = () => {
  return (
    <div>
      <Table name="brand" />
    </div>
  );
};

export default App;
