import CommonDB from "./CommonDB";
import { ImageEntry } from "../logic/PictureEvolutionLogic";

const pictureEntries: string = "picture_entries";
const colId: string = "id";
const colDate: string = "date";
const colImage: string = "image";

export const createImageTable = () => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${pictureEntries} (
      ${colId} INTEGER PRIMARY KEY AUTOINCREMENT,
      ${colDate} DATE NOT NULL UNIQUE,
      ${colImage} TEXT NOT NULL);`
    );
  });
};

export const getImageEntries = (): Promise<ImageEntry[]> => {
  return new Promise<ImageEntry[]>((resolve, _) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(
        `SELECT * FROM ${pictureEntries} ORDER BY ${colDate} ASC;`,
        [],
        (_, result) => {
          const imageEntries: ImageEntry[] = [];
          for (let i = 0; i < result.rows.length; i++) {
            const row = result.rows.item(i);
            imageEntries.push({
              id: row[colId],
              date: new Date(row[colDate]),
              image: row[colImage],
            });
          }
          resolve(imageEntries);
        }
      );
    });
  });
};

export const addImageEntry = (date: string, image: string) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `INSERT INTO ${pictureEntries} (${colDate}, ${colImage}) VALUES (?, ?);`,
      [date, image]
    );
  });
};
