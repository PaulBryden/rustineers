import("../pkg").then(module => {
    console.log("Hello From Typescript! MODIFIED")
    module.print_function();
  });