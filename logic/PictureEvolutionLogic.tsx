import { addImageEntry, getImageEntries } from "../database/PictureEvolutionDB";

export interface ImageEntry {
  id: number;
  image: string;
  date: Date;
}

export function refreshImageEntries(
  setImageList: (
    value: ((prevState: ImageEntry[]) => ImageEntry[]) | ImageEntry[]
  ) => void
) {
  getImageEntries().then((ie: ImageEntry[]) => {
    setImageList(ie);
  });
}

export function addImage(uri: string) {
  addImageEntry(new Date().toISOString().split("T")[0], uri);
}
