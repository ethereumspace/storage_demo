import { storage_example } from "../../declarations/storage_example";

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const name = document.getElementById("name").value.toString();
  // Interact with storage_example actor, calling the greet method
  const greeting = await storage_example.greet(name);

  document.getElementById("greeting").innerText = greeting;
});
