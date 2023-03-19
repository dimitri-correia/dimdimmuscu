import CommonDB from "./CommonDB";
import { CardioEntry } from "../logic/CardioLogic";

const cardioEntries = "cardio_entries";
const colId = "id";
const colDate = "date";
const colName = "name";
const colTime = "time";
const colCalo = "calo";

interface tableEntry {
  id: number;
  date: string;
  name: string;
  time: number;
  calo: number;
}

export const createCardioTable = () => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${cardioEntries} (
      ${colId} INTEGER PRIMARY KEY AUTOINCREMENT,
      ${colDate} DATE NOT NULL UNIQUE,
      ${colName} TEXT NOT NULL),
      ${colTime} REAL NOT NULL),
      ${colCalo} REAL NOT NULL);`
    );
  });
};

export const getCardioEntries = (): Promise<CardioEntry[]> => {
  return new Promise<CardioEntry[]>((resolve) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(
        `SELECT * FROM ${cardioEntries} ORDER BY ${colId} ASC;`,
        [],
        (_, result) => {
          const cardioEntries: CardioEntry[] = [];
          for (let i = 0; i < result.rows.length; i++) {
            const { id, date, name, time, calo } = result.rows.item(
              i
            ) as tableEntry;
            cardioEntries.push({
              id: id,
              date: new Date(date),
              name: name,
              time: time,
              calo: calo,
            });
          }
          resolve(cardioEntries);
        }
      );
    });
  });
};

export const addCardioEntry = (
  date: string,
  name: string,
  time: number,
  calo: number
) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `INSERT INTO ${cardioEntries} (${colDate}, ${colName} ${colTime} ${colCalo}) VALUES (?, ?, ?, ?);`,
      [date, name, time, calo]
    );
  });
};
