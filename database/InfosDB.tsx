import CommonDB from "./CommonDB";
import { InfoEntry } from "../logic/InfosLogic";

const infoEntries = "info_entries";
const colId = "id";
const colFieldName = "field_name"; // only useful when looking directly in the db
const colFieldValue = "field_value";

export const createInfosTable = () => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${infoEntries} (
      ${colId} INTEGER PRIMARY KEY AUTOINCREMENT,
      ${colFieldName} STRING NOT NULL UNIQUE,
      ${colFieldValue} STRING NOT NULL);`
    );
  });
};

export const getInfoEntries = (): Promise<Map<number, InfoEntry>> => {
  return new Promise<Map<number, InfoEntry>>((resolve, _) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(`SELECT * FROM ${infoEntries};`, [], (_, result) => {
        const infoEntries: Map<number, InfoEntry> = new Map();
        for (let i = 0; i < result.rows.length; i++) {
          const row = result.rows.item(i);
          infoEntries.set(row[colId], {
            fieldName: row[colFieldName],
            fieldValue: row[colFieldValue],
          });
        }
        resolve(infoEntries);
      });
    });
  });
};

export const editInfosEntry = (id: number, value: string) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `UPDATE ${infoEntries} SET ${colFieldValue} = ? WHERE ${colId} = ?;`,
      [value, id]
    );
  });
};
