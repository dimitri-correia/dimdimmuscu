import { StyleSheet } from "react-native";

export default StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: "white",
    paddingHorizontal: 10,
    paddingTop: 20,
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
    marginHorizontal: "25%",
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
    borderRadius: 8,
    marginLeft: 10,
    paddingHorizontal: 12,
    paddingVertical: 8,
    alignItems: "center",
    justifyContent: "center",
  },
  buttonOK: {
    backgroundColor: "#007AFF",
  },
  buttonKO: {
    backgroundColor: "#FF3B30",
  },
  buttonText: {
    color: "#FFFFFF",
    fontWeight: "bold",
    textAlign: "center",
  },
});
