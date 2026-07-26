(() => {
  const filename = window.location.pathname.split("/").pop() || "";
  const match = filename.match(/^(\d{2})[A-Z]?_[^/]+\.html$/);
  if (!match) return;

  const chapter = Number(match[1]);
  const part =
    chapter <= 4 ? 0 :
    chapter <= 12 ? 1 :
    chapter <= 20 ? 2 :
    chapter <= 26 ? 3 :
    chapter <= 30 ? 4 :
    chapter <= 35 ? 5 :
    chapter <= 40 ? 6 : 7;

  const heading = document.querySelector("main h1");
  if (!heading) return;

  const link = document.createElement("a");
  link.className = "full-source-link";
  link.href = `source/part${part}.html`;
  link.textContent = `⌨ Part ${part} 전체 코드 보기`;
  link.setAttribute("aria-label", `${chapter}장 예제가 포함된 Part ${part} 전체 코드 보기`);
  heading.insertAdjacentElement("afterend", link);
})();
