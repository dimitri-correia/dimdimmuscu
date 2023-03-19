import CommonDB from "./CommonDB";
import { SessionTrackerEntry } from "../logic/SessionTrackerLogic";

const liftEntries = "lift_entries";
const colId = "id";
const colDate = "date";
const colName = "name";
const colSet = "set";
const colRep = "rep";
const colWeight = "weight";

interface tableEntry {
  id: number;
  date: string;
  name: string;
  set: number;
  rep: number;
  weight: number;
}

export const createSessionTrackerTable = () => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${liftEntries} (
      ${colId} INTEGER PRIMARY KEY AUTOINCREMENT,
      ${colDate} DATE NOT NULL UNIQUE,
      ${colName} TEXT NOT NULL),
      ${colSet} REAL NOT NULL),
      ${colRep} REAL NOT NULL),
      ${colWeight} REAL NOT NULL);`
    );
  });
};

export const getSessionTrackerEntries = (): Promise<SessionTrackerEntry[]> => {
  return new Promise<SessionTrackerEntry[]>((resolve) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(
        `SELECT * FROM ${liftEntries} ORDER BY ${colId} ASC;`,
        [],
        (_, result) => {
          const sessionTrackerEntries: SessionTrackerEntry[] = [];
          for (let i = 0; i < result.rows.length; i++) {
            const { id, date, name, set, rep, weight } = result.rows.item(
              i
            ) as tableEntry;
            sessionTrackerEntries.push({
              id: id,
              date: new Date(date),
              name: name,
              set: set,
              rep: rep,
              weight: weight,
            });
          }
          resolve(sessionTrackerEntries);
        }
      );
    });
  });
};

export const addSessionTrackerEntry = (
  date: string,
  name: string,
  set: number,
  rep: number,
  weight: number
) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `INSERT INTO
      ${liftEntries} (${colDate}, ${colName} ${colSet} ${colRep} ${colWeight})
      VALUES (?, ?, ?, ?, ?);`,
      [date, name, set, rep, weight]
    );
  });
};
