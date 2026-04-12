     1|// Type declarations for CSS modules and CSS imports
     2|declare module '*.css' {
     3|  const content: string;
     4|  export default content;
     5|}
     6|
     7|declare module '*.module.css' {
     8|  const classes: { readonly [key: string]: string };
     9|  export default classes;
    10|}
    11|