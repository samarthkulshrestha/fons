async function search(prompt) {
  const results = document.getElementById("results")
  results.innerHTML = "";
  const response = await fetch("/" + prompt, {
    method: 'GET',
  });
  const json = await response.json();
  results.innerHTML = "";

  let def = document.createElement("span");
  def.appendChild(document.createTextNode(json[0]["word"]));
  def.appendChild(document.createTextNode(" "));
  def.appendChild(document.createTextNode(json[0]["pos"]));
  def.appendChild(document.createTextNode(" "));
  def.appendChild(document.createTextNode(json[0]["ipa"]));
  def.appendChild(document.createElement("br"));
  def.appendChild(document.createTextNode("wiktionary: "));
  def.appendChild(document.createTextNode(json[0]["etym_txt_wiki"]));
  def.appendChild(document.createElement("br"));
  def.appendChild(document.createTextNode("etymonline: "));
  def.appendChild(document.createTextNode(json[0]["etym_txt_etym"]));
  def.appendChild(document.createElement("br"));
  results.appendChild(def);

  // for (let [key, obj] of Object.entries(json)) {
  //     console.log(key);
  //     console.log(obj);
  //     let item = document.createElement("span");
  //     item.appendChild(document.createTextNode(obj["word"]));
  //     item.appendChild(document.createTextNode(" "));
  //     item.appendChild(document.createTextNode(obj["pos"]));
  //     item.appendChild(document.createTextNode(" "));
  //     item.appendChild(document.createTextNode(obj["ipa"]));
  //     item.appendChild(document.createElement("br"));
  //     item.appendChild(document.createTextNode("wiktionary: "));
  //     item.appendChild(document.createTextNode(obj["etym_txt_wiki"]));
  //     item.appendChild(document.createElement("br"));
  //     item.appendChild(document.createTextNode("etymonline: "));
  //     item.appendChild(document.createTextNode(obj["etym_txt_etym"]));
  //     item.appendChild(document.createElement("br"));
  //     item.appendChild(document.createElement("br"));
  //     results.appendChild(item);
  // }
}

let query = document.getElementById("query");
let currentSearch = Promise.resolve()

query.addEventListener("keypress", (e) => {
  if (e.key == "Enter") {
    currentSearch.then(() => search(query.value));
  }
})
