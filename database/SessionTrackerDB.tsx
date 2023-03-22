import CommonDB from "./CommonDB";
import {
  SessionTrackerLiftEntry,
  SessionTrackerSetEntry,
} from "../logic/SessionTrackerLogic";

const liftEntries = "lift_entries";
const colIdSession = "id";
const colDate = "date";
const colEx = "ex";

const setEntries = "set_entries";
const colIdSet = "id";
const colSet = "set";
const colRep = "rep";
const colWeight = "weight";
const colRefLift = "liftId";

interface tableEntryLift {
  id: number;
  date: string;
  name: string;
}

interface tableEntrySet {
  id: number;
  idLift: number;
  set: number;
  rep: number;
  weight: number;
}

export const createSessionTrackerTable = () => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${liftEntries} (
      ${colIdSession} INTEGER PRIMARY KEY AUTOINCREMENT,
      ${colDate} DATE NOT NULL UNIQUE,
      ${colEx} TEXT NOT NULL);`
    );
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${setEntries} (
      ${colIdSet} INTEGER PRIMARY KEY AUTOINCREMENT,
      ${colSet} REAL NOT NULL,
      ${colRep} REAL NOT NULL,
      ${colWeight} REAL NOT NULL,
      ${colRefLift} INTEGER NOT NULL,
      FOREIGN KEY (${colRefLift}) REFERENCES ${liftEntries}(${colIdSession})
      );`
    );
  });
};

export const getSessionTrackerLiftEntries = (): Promise<
  SessionTrackerLiftEntry[]
> => {
  return new Promise<SessionTrackerLiftEntry[]>((resolve) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(
        `SELECT * FROM ${liftEntries} ORDER BY ${colIdSession} ASC;`,
        [],
        (_, result) => {
          const sessionTrackerEntries: SessionTrackerLiftEntry[] = [];
          for (let i = 0; i < result.rows.length; i++) {
            const { id, date, name } = result.rows.item(i) as tableEntryLift;
            sessionTrackerEntries.push({
              id: id,
              date: new Date(date),
              name: name,
            });
          }
          resolve(sessionTrackerEntries);
        }
      );
    });
  });
};
export const getSessionTrackerSetEntries = (
  idLiftReferenced: number
): Promise<SessionTrackerSetEntry[]> => {
  return new Promise<SessionTrackerSetEntry[]>((resolve) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(
        `SELECT * FROM ${setEntries} WHERE ${colRefLift} = ? ORDER BY ${colSet} ASC;`,
        [idLiftReferenced],
        (_, result) => {
          const sessionTrackerEntries: SessionTrackerSetEntry[] = [];
          for (let i = 0; i < result.rows.length; i++) {
            const { id, set, rep, weight } = result.rows.item(
              i
            ) as tableEntrySet;
            sessionTrackerEntries.push({
              id: id,
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
