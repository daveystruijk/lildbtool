import { Component, For, Suspense, createResource } from "solid-js";

import type { TableDetailResult } from "../../backend/bindings/TableDetailResult";
import { API_URL } from "./config";

const fetchTableDetails = async (table_name: string) => {
  const result = await fetch(`${API_URL}/tables/${table_name}`);
  return result.json() as Promise<TableDetailResult[]>;
};

const TableView: Component<{ table_name: string }> = (props) => {
  const [tableDetails] = createResource(props.table_name, fetchTableDetails);

  return (
    <Suspense fallback={<div>Loading...</div>}>
      <table class="table-auto">
        <thead>
          <tr>
            <For each={tableDetails()}>
              {(tableColumn) => <th>{tableColumn.column_name}</th>}
            </For>
          </tr>
        </thead>
        <tbody>
          <tr>
            <For each={tableDetails()}>
              {(tableColumn) => <td>{tableColumn.data_type}</td>}
            </For>
          </tr>
        </tbody>
      </table>
    </Suspense>
  );
};

export default TableView;
