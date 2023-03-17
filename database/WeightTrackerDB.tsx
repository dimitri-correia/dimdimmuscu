import CommonDB from "./CommonDB";
import { WeightEntry } from "../logic/WeightTrackerLogic";

const weightEntries = "weight_entries";
const colId = "id";
const colDate = "date";
const colWeight = "weight";

interface tableEntry {
  id: number;
  date: string;
  weight: number;
}

export const createWeightTable = () => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${weightEntries} (
      ${colId} INTEGER PRIMARY KEY AUTOINCREMENT,
      ${colDate} DATE NOT NULL UNIQUE,
      ${colWeight} REAL NOT NULL);`
    );
  });
};

export const getWeightEntries = (): Promise<WeightEntry[]> => {
  return new Promise<WeightEntry[]>((resolve) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(
        `SELECT * FROM ${weightEntries} ORDER BY ${colId} ASC;`,
        [],
        (_, result) => {
          const weightEntries: WeightEntry[] = [];
          for (let i = 0; i < result.rows.length; i++) {
            const { id, date, weight } = result.rows.item(i) as tableEntry;
            weightEntries.push({
              id: id,
              date: new Date(date),
              weight: weight,
            });
          }
          resolve(weightEntries);
        }
      );
    });
  });
};

export const getLastWeight = (): Promise<WeightEntry> => {
  return new Promise<WeightEntry>((resolve, _) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(
        `SELECT * FROM ${weightEntries} ORDER BY ${colDate} DESC LIMIT 1;`,
        [],
        (_, result) => {
          const { id, date, weight } = result.rows.item(0) as tableEntry;
          resolve({
            id: id,
            date: new Date(date),
            weight: weight,
          });
        }
      );
    });
  });
};

export const addWeightEntry = (date: string, weight: number) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `INSERT INTO ${weightEntries} (${colDate}, ${colWeight}) VALUES (?, ?);`,
      [date, weight]
    );
  });
};

export const editWeightEntry = (id: number, weight: number) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `UPDATE ${weightEntries} SET ${colWeight} = ? WHERE ${colId} = ?;`,
      [weight, id]
    );
  });
};

export const deleteWeightEntry = (id: number) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(`DELETE FROM ${weightEntries} WHERE ${colId} = ?;`, [id]);
  });
};
