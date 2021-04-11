if(document.querySelector("table")){
    document.querySelector("table").innerHTML = document.querySelector("table").innerHTML.split("§").map(x => "<td>" + x + "</td>").join("")
}

document.querySelectorAll("#submit_todo").forEach(elem => {
    elem.addEventListener("click", _ =>{
        const inputfields = ["name", "expected_duration", "priority"]
        const out_obj = {}
        for(const field of inputfields)
            out_obj[field] = document.querySelector("#" + field).value
        out_obj.user_id = document.querySelector("#user_id").innerHTML.split(":")[1]
        out_obj.id      = document.querySelector("#id"     ).innerHTML.split(":")[1]

        const xhr = new XMLHttpRequest();
        xhr.open("POST", "/todoadd", true); 
        xhr.setRequestHeader("Content-Type", "application/json");
        xhr.send(JSON.stringify(out_obj).replaceAll(/id":"(.*?)"/g, x => x.slice(0, 4) + x.split(":")[1].slice(1).split('"')[0]))
        window.location.replace(window.location.href)
    })
})

document.querySelectorAll("#submit_activity").forEach(elem => {
    elem.addEventListener("click", _ =>{
        const inputfields = ["type", "start_time", "duration", "satisfaction"]
        const out_obj = {}
        for(const field of inputfields)
            out_obj[field] = document.querySelector("#" + field).value
        out_obj.user_id = document.querySelector("#user_id").innerHTML.split(":")[1]
        out_obj.id      = document.querySelector("#id"     ).innerHTML.split(":")[1]

        const xhr = new XMLHttpRequest();
        xhr.open("POST", "/activityadd", true); 
        xhr.setRequestHeader("Content-Type", "application/json");
        xhr.send(JSON.stringify(out_obj).replaceAll(/id":"(.*?)"/g, x => x.slice(0, 4) + x.split(":")[1].slice(1).split('"')[0]))
        window.location.replace(window.location.href)
    })
})
