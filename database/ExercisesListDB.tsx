import CommonDB from "./CommonDB";
import { ExercisesEntry } from "../logic/ExercisesListLogic";

export const exercisesEntriesDBName = "exercises_entries";
export const colExIdExercisesEntriesDBName = "id";
const exExerciseName = "exercise_name";
const exPrimaryMuscleGroup = "primary_muscle";
const exSecondaryMuscleGroup = "secondary_muscle";

const muscleGroupEntriesDBName = "muscle_group_entries";
const muscleId = "id";
const muscleMuscleGroupName = "muscle_group_name";

interface tableEntryEx {
  id: number;
  name: string;
  primary: number;
  secondary: number;
}

interface tableEntryMG {
  id: number;
  name: string;
}

export const createExercisesTableS = () => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${muscleGroupEntriesDBName} (
      ${muscleId} INTEGER PRIMARY KEY AUTOINCREMENT,
      ${muscleMuscleGroupName} TEXT NOT NULL UNIQUE);`
    );
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${exercisesEntriesDBName} (
      ${colExIdExercisesEntriesDBName} INTEGER PRIMARY KEY AUTOINCREMENT,
      ${exExerciseName} TEXT NOT NULL UNIQUE,
      ${exPrimaryMuscleGroup} INTEGER NOT NULL,
      ${exSecondaryMuscleGroup} INTEGER,
      FOREIGN KEY (${exPrimaryMuscleGroup}) REFERENCES ${muscleGroupEntriesDBName}(${muscleId}),
      FOREIGN KEY (${exSecondaryMuscleGroup}) REFERENCES ${muscleGroupEntriesDBName}(${muscleId})
  );`
    );
    tx.executeSql(
      `INSERT OR IGNORE INTO ${muscleGroupEntriesDBName} (${muscleId}, ${muscleMuscleGroupName}) 
       VALUES (0, ".");`
    );
  });
};

export const getMuscleGroupEntries = (): Promise<Map<number, string>> => {
  return new Promise<Map<number, string>>((resolve) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(
        `SELECT * FROM ${muscleGroupEntriesDBName} ORDER BY LOWER(${muscleMuscleGroupName});`,
        [],
        (_, result) => {
          const muscleGroupEntries = new Map<number, string>();
          for (let i = 0; i < result.rows.length; i++) {
            const { id, name } = result.rows.item(i) as tableEntryMG;
            muscleGroupEntries.set(id, name);
          }
          resolve(muscleGroupEntries);
        }
      );
    });
  });
};

export const getExerciseEntries = (): Promise<ExercisesEntry[]> => {
  return new Promise<ExercisesEntry[]>((resolve) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(
        `SELECT * FROM ${exercisesEntriesDBName}
        ORDER BY LOWER(${exPrimaryMuscleGroup}), LOWER(${exSecondaryMuscleGroup}),
        LOWER(${exExerciseName});`,
        [],
        (_, result) => {
          const exercisesEntries: ExercisesEntry[] = [];
          for (let i = 0; i < result.rows.length; i++) {
            const { id, name, primary, secondary } = result.rows.item(
              i
            ) as tableEntryEx;
            exercisesEntries.push({
              id: id,
              name: name,
              primaryMuscleGroupId: primary,
              secondaryMuscleGroupId: secondary,
            });
          }
          resolve(exercisesEntries);
        }
      );
    });
  });
};

export const addMuscleGroupEntry = (name: string) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `INSERT INTO ${muscleGroupEntriesDBName} (${muscleMuscleGroupName}) VALUES (?);`,
      [name]
    );
  });
};

export const addExerciseEntry = (
  name: string,
  primaryMuscleGroupId: number,
  secondaryMuscleGroupId: number
) => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `INSERT INTO ${exercisesEntriesDBName} (${exExerciseName},${exPrimaryMuscleGroup},${exSecondaryMuscleGroup}) VALUES (?,?,?);`,
      [name, primaryMuscleGroupId, secondaryMuscleGroupId]
    );
  });
};
