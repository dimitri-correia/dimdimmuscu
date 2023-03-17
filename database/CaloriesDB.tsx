import CommonDB from "./CommonDB";
import { CaloEntry } from "../logic/CaloriesTrackerLogic";

const caloEntries = "calories_entries";
const colId = "id";
const colDate = "date";
const colFoodName = "food_name";
const colFoodCalories = "food_calo";
const colFoodProteins = "food_prot";

interface tableEntry {
  id: number;
  date: string;
  name: string;
  calo: number;
  prot: number;
}

export const createCaloriesTable = () => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${caloEntries} (
      ${colId} INTEGER PRIMARY KEY AUTOINCREMENT,
      ${colDate} DATE NOT NULL UNIQUE,
      ${colFoodName} TEXT NOT NULL,
      ${colFoodCalories} REAL NOT NULL,
      ${colFoodProteins} REAL NOT NULL);`
    );
  });
};

const getCaloEntries = (sql: string): Promise<CaloEntry[]> => {
  return new Promise<CaloEntry[]>((resolve) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(sql, [], (_, result) => {
        const caloEntries: CaloEntry[] = [];
        for (let i = 0; i < result.rows.length; i++) {
          const { id, date, name, calo, prot } = result.rows.item(
            i
          ) as tableEntry;
          caloEntries.push({
            id: id,
            date: new Date(date),
            name: name,
            calo: calo,
            prot: prot,
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

export const addCaloEntry = (
  date: string,
  name: string,
  calo: number,
  prot: number
) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `INSERT INTO ${caloEntries} (${colDate}, ${colFoodName}, ${colFoodCalories}, ${colFoodProteins}) VALUES (?, ?, ?, ?);`,
      [date, name, calo, prot]
    );
  });
};

export const editWeightEntry = (
  id: number,
  name: string,
  calo: number,
  prot: number
) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `UPDATE ${caloEntries} SET (${colFoodName}, ${colFoodCalories}, ${colFoodProteins}) = (?, ?, ?) WHERE ${colId} = ?;`,
      [name, calo, prot, id]
    );
  });
};

export const deleteWeightEntry = (id: number) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(`DELETE FROM ${caloEntries} WHERE ${colId} = ?;`, [id]);
  });
};
