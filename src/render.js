const closeButton = document.getElementById("exit");
// // Called when message received from main process
// api.receiveFromD((event, data) => {
//   console.log(event, data);
//   console.log(`Received ${data} from main process`);
// });

// // Send a message to the main process
// api.sendToA();

closeButton.addEventListener("click", function () {
  api.requestWindowClose();
});
