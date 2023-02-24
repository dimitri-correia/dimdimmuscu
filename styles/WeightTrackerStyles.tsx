import { StyleSheet } from "react-native";

export const pageStyles = StyleSheet.create({
  row: {
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "center",
    paddingVertical: 8,
    paddingHorizontal: 16,
    borderBottomWidth: 1,
    borderBottomColor: "#ccc",
    width: "100%",
  },
  column: {
    flex: 1,
    textAlign: "center",
    color: "blue",
    fontSize: 16,
  },
});

export const addWeightStyles = StyleSheet.create({
  addWeightContainer: {
    flexDirection: "row",
    alignItems: "center",
    marginBottom: 16,
  },
  addWeightLabel: {
    fontSize: 16,
    marginRight: 8,
  },
  addWeightInput: {
    flex: 1,
    borderWidth: 1,
    borderColor: "#ccc",
    borderRadius: 4,
    padding: 8,
    marginRight: 8,
    backgroundColor: "#444",
    color: "#fff",
  },
});

export const editWeightStyles = StyleSheet.create({
  modal: {
    flex: 1,
    alignItems: "center",
    justifyContent: "center",
  },
  background: {
    backgroundColor: "grey",
  },
  line: {
    flexDirection: "row",
    alignItems: "center",
    marginVertical: "5%",
    marginHorizontal: "15%",
  },
  title: {
    fontSize: 20,
    fontWeight: "bold",
  },
  label: {
    flex: 1,
    fontWeight: "bold",
    marginRight: 16,
  },
  input: {
    flex: 1,
    backgroundColor: "#444",
    color: "#fff",
    borderWidth: 2,
    borderColor: "#ccc",
    borderRadius: 4,
    padding: 8,
    marginRight: 8,
  },
  button: {
    flex: 1,
    backgroundColor: "#007AFF",
    borderRadius: 8,
    marginLeft: 10,
    paddingHorizontal: 12,
    paddingVertical: 8,
    alignItems: "center",
    justifyContent: "center",
  },
  buttonText: {
    color: "#FFFFFF",
    fontWeight: "bold",
    textAlign: "center",
  },
});
