import { StyleSheet } from "react-native";

export const OneRmStyles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: "center",
    justifyContent: "center",
    paddingHorizontal: 20,
  },
  label: {
    fontWeight: "bold",
  },
  input: {
    borderWidth: 1,
    borderColor: "#ccc",
    padding: 10,
    borderRadius: 10,
    marginBottom: 20,
    alignSelf: "stretch",
  },
  button: {
    backgroundColor: "#2196F3",
    padding: 15,
    borderRadius: 5,
    alignSelf: "stretch",
  },
  buttonText: {
    color: "#fff",
    fontWeight: "bold",
    textAlign: "center",
  },
  result: {
    marginTop: 10,
  },
});

export const maxAttemptStyles = StyleSheet.create({
  container: {
    padding: 16,
    flex: 1,
  },
  title: {
    fontWeight: "bold",
  },
  input: {
    borderWidth: 1,
    borderColor: "#ccc",
    padding: 10,
    borderRadius: 10,
    marginBottom: 20,
    alignSelf: "stretch",
  },
  table: {
    flex: 1,
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
  },
  heading: {
    fontWeight: "bold",
    margin: 4,
    width: "20%",
    textAlign: "center",
  },
  row: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    margin: 4,
    width: "100%",
  },
  cell: {
    margin: 4,
    width: "20%",
    textAlign: "center",
  },
});
