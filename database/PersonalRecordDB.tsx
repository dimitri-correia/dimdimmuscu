import CommonDB from "./CommonDB";
import { PersonalRecordEntry } from "../logic/PersonalRecordLogic";

const personalRecordDBName = "personal_record";
const colId = "id";
const colDate = "date";
const colEx = "exo";
const colWeightLifted = "weight";

export const createPRTable = () => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${personalRecordDBName} (
      ${colId} INTEGER PRIMARY KEY AUTOINCREMENT,
      ${colDate} DATE NOT NULL,
      ${colEx} INTEGER NOT NULL,
      ${colWeightLifted} REAL NOT NULL );`
    );
  });
};

export const getPREntries = (): Promise<PersonalRecordEntry[]> => {
  return new Promise<PersonalRecordEntry[]>((resolve, _) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(
        `SELECT * FROM ${personalRecordDBName} ORDER BY ${colEx}, ${colDate};`,
        [],
        (_, result) => {
          const personalRecordEntries: PersonalRecordEntry[] = [];
          for (let i = 0; i < result.rows.length; i++) {
            const row = result.rows.item(i);
            personalRecordEntries.push({
              id: row[colId],
              date: new Date(row[colDate]),
              exId: row[colEx],
              weightLifted: row[colWeightLifted],
            });
          }
          resolve(personalRecordEntries);
        }
      );
    });
  });
};

export const addPREntry = (date: string, exoID: number, weight: number) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `INSERT INTO ${personalRecordDBName} (${colDate}, ${colEx} ${colWeightLifted}) VALUES (?, ?, ?);`,
      [date, exoID, weight]
    );
  });
};

export const editPREntry = (id: number, weight: number) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `UPDATE ${personalRecordDBName} SET ${colWeightLifted} = ? WHERE ${colId} = ?;`,
      [weight, id]
    );
  });
};

export const deletePREntry = (id: number) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(`DELETE FROM ${personalRecordDBName} WHERE ${colId} = ?;`, [
      id,
    ]);
  });
};
