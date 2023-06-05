import type { Component } from "solid-js";

import type { TableResult } from "../../backend/bindings/TableResult";
import TableView from "./TableView";

const App: Component = () => {
  return (
    <div>
      <TableView table_name="brand" />
    </div>
  );
};

export default App;
