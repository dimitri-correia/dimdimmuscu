import CommonDB from "./CommonDB";
import { WeightEntry } from "../logic/WeightTrackerLogic";

export const createWeightTable = () => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS weight_entries (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      date DATE NOT NULL UNIQUE,
      weight REAL NOT NULL);`
    );
  });
};

export const getWeightEntries = (): Promise<WeightEntry[]> => {
  return new Promise<WeightEntry[]>((resolve, _) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(
        "SELECT * FROM weight_entries ORDER BY date DESC;",
        [],
        (_, result) => {
          const weightEntries: WeightEntry[] = [];
          for (let i = 0; i < result.rows.length; i++) {
            const row = result.rows.item(i);
            weightEntries.push({
              id: row.id,
              date: new Date(row.date),
              weight: row.weight,
            });
          }
          resolve(weightEntries);
        }
      );
    });
  });
};

export const addWeightEntry = (date: Date, weight: number) => {
  CommonDB.transaction((tx) => {
    tx.executeSql("INSERT INTO weight_entries (date, weight) VALUES (?, ?);", [
      date.toISOString().split("T")[0], //YYYY-MM-DDTHH:mm:ss - > YYYY-MM-DD
      weight,
    ]);
  });
};
export const editWeightEntry = (id: number, weight: number) => {
  CommonDB.transaction((tx) => {
    tx.executeSql("UPDATE weight_entries SET weight = ? WHERE id = ?;", [
      weight,
      id,
    ]);
  });
};
export const deleteWeightEntry = (id: number) => {
  CommonDB.transaction((tx) => {
    tx.executeSql("DELETE FROM weight_entries WHERE id = ?;", [id]);
  });
};
