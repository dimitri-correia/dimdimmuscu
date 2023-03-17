import CommonDB from "./CommonDB";
import { InfoEntry } from "../logic/InfosLogic";

const infoEntries = "info_entries";
const colId = "id";
const colFieldName = "field_name"; // only useful when looking directly in the db
const colFieldValue = "field_value";

interface tableEntry {
  id: number;
  name: string;
  value: string;
}

export const createInfosTable = () => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${infoEntries} (
        ${colId} INTEGER PRIMARY KEY AUTOINCREMENT,
        ${colFieldName} TEXT NOT NULL UNIQUE,
        ${colFieldValue} TEXT NOT NULL);`,
      [],
      () => {
        // Insert default values if they don't already exist
        tx.executeSql(
          `INSERT INTO ${infoEntries} (${colId}, ${colFieldName}, ${colFieldValue}) 
          SELECT 1, 'name', 'dim' 
          WHERE NOT EXISTS (SELECT 1 FROM ${infoEntries} WHERE ${colId} = 1);`,
          [],
          () => {
            tx.executeSql(
              `INSERT INTO ${infoEntries} (${colId}, ${colFieldName}, ${colFieldValue}) 
              SELECT 2, 'birthdate', '05-05-1999' 
              WHERE NOT EXISTS (SELECT 1 FROM ${infoEntries} WHERE ${colId} = 2);`,
              [],
              () => {
                tx.executeSql(
                  `INSERT INTO ${infoEntries} (${colId}, ${colFieldName}, ${colFieldValue}) 
                  SELECT 3, 'height', '180' 
                  WHERE NOT EXISTS (SELECT 1 FROM ${infoEntries} WHERE ${colId} = 3);`,
                  []
                );
              }
            );
          }
        );
      }
    );
  });
};

export const getInfoEntries = (): Promise<Map<number, InfoEntry>> => {
  return new Promise<Map<number, InfoEntry>>((resolve) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(`SELECT * FROM ${infoEntries};`, [], (_, result) => {
        const infoEntries: Map<number, InfoEntry> = new Map();
        for (let i = 0; i < result.rows.length; i++) {
          const { id, name, value } = result.rows.item(i) as tableEntry;
          infoEntries.set(id, {
            fieldName: name,
            fieldValue: value,
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
