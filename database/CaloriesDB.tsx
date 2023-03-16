import CommonDB from "./CommonDB";
import { CaloEntry } from "../logic/CaloriesTrackerLogic";

const caloEntries = "calories_entries";
const colId = "id";
const colDate = "date";
const colFoodName = "food_name";
const colFoodCalories = "food_calo";

export const createCaloriesTable = () => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${caloEntries} (
      ${colId} INTEGER PRIMARY KEY AUTOINCREMENT,
      ${colDate} DATE NOT NULL UNIQUE,
      ${colFoodName} TEXT NOT NULL,
      ${colFoodCalories} REAL NOT NULL);`
    );
  });
};

const getCaloEntries = (sql: string): Promise<CaloEntry[]> => {
  return new Promise<CaloEntry[]>((resolve, _) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(sql, [], (_, result) => {
        const caloEntries: CaloEntry[] = [];
        for (let i = 0; i < result.rows.length; i++) {
          const { id, date, name, calo } = result.rows.item(i);
          caloEntries.push({
            id: id,
            date: new Date(date),
            name: name,
            calo: calo,
          });
        }
        resolve(caloEntries);
      });
    });
  });
};

export const getCaloEntriesAll = (): Promise<CaloEntry[]> => {
  return getCaloEntries(
    `SELECT * FROM ${caloEntries} ORDER BY ${colDate} ASC;`
  );
};
export const getCaloEntriesPreciseDate = (
  date: string
): Promise<CaloEntry[]> => {
  return getCaloEntries(
    `SELECT * FROM ${caloEntries} WHERE ${colDate} = ${date};`
  );
};

export const addCaloEntry = (date: string, name: string, calo: number) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `INSERT INTO ${caloEntries} (${colDate}, ${colFoodName}, ${colFoodCalories}) VALUES (?, ?, ?);`,
      [date, name, calo]
    );
  });
};

export const editWeightEntry = (id: number, name: string, calo: number) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `UPDATE ${caloEntries} SET (${colFoodName}, ${colFoodCalories}) = (?, ?) WHERE ${colId} = ?;`,
      [name, calo, id]
    );
  });
};

export const deleteWeightEntry = (id: number) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(`DELETE FROM ${caloEntries} WHERE ${colId} = ?;`, [id]);
  });
};
