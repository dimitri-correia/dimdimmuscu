import CommonDB from "./CommonDB";
import { ExercisesEntry } from "../logic/ExercisesListLogic";

const exercisesEntriesDBName: string = "exercises_entries";
const exId: string = "id";
const exExerciseName: string = "exercise_name";
const exPrimaryMuscleGroup: string = "primary_muscle";
const exSecondaryMuscleGroup: string = "secondary_muscle";

const muscleGroupEntriesDBName: string = "muscle_group_entries";
const muscleId: string = "id";
const muscleMuscleGroupName: string = "muscle_group_name";

export const createExercisesTableS = () => {
  CommonDB.transaction((tx) => {
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${muscleGroupEntriesDBName} (
      ${muscleId} INTEGER PRIMARY KEY AUTOINCREMENT,
      ${muscleMuscleGroupName} TEXT NOT NULL UNIQUE);`
    );
    tx.executeSql(
      `CREATE TABLE IF NOT EXISTS ${exercisesEntriesDBName} (
      ${exId} INTEGER PRIMARY KEY AUTOINCREMENT,
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
  return new Promise<Map<number, string>>((resolve, _) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(
        `SELECT * FROM ${muscleGroupEntriesDBName} ORDER BY LOWER(${muscleMuscleGroupName});`,
        [],
        (_, result) => {
          const muscleGroupEntries = new Map<number, string>();
          for (let i = 0; i < result.rows.length; i++) {
            const row = result.rows.item(i);
            muscleGroupEntries.set(row[muscleId], row[muscleMuscleGroupName]);
          }
          resolve(muscleGroupEntries);
        }
      );
    });
  });
};

export const getExerciseEntries = (): Promise<ExercisesEntry[]> => {
  return new Promise<ExercisesEntry[]>((resolve, _) => {
    CommonDB.transaction((tx) => {
      tx.executeSql(
        `SELECT * FROM ${exercisesEntriesDBName}
        ORDER BY LOWER(${exPrimaryMuscleGroup}), LOWER(${exSecondaryMuscleGroup}),
        LOWER(${exExerciseName});`,
        [],
        (_, result) => {
          const exercisesEntries: ExercisesEntry[] = [];
          for (let i = 0; i < result.rows.length; i++) {
            const row = result.rows.item(i);
            exercisesEntries.push({
              id: row[exId],
              name: row[exExerciseName],
              primaryMuscleGroupId: row[exPrimaryMuscleGroup],
              secondaryMuscleGroupId: row[exSecondaryMuscleGroup],
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
