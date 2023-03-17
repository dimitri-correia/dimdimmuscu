import CommonDB from "./CommonDB";
import { ImageEntry } from "../logic/PictureEvolutionLogic";

const pictureEntries = "picture_entries";
const colId = "id";
const colDate = "date";
const colImage = "image";

interface tableEntry {
  id: number;
  date: string;
  image: string;
}

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
  return new Promise<ImageEntry[]>((resolve) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(
        `SELECT * FROM ${pictureEntries} ORDER BY ${colDate} ASC;`,
        [],
        (_, result) => {
          const imageEntries: ImageEntry[] = [];
          for (let i = 0; i < result.rows.length; i++) {
            const { id, date, image } = result.rows.item(i) as tableEntry;
            imageEntries.push({
              id: id,
              date: new Date(date),
              image: image,
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
