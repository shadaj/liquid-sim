(window.webpackJsonp=window.webpackJsonp||[]).push([[1],[,function(t,n,e){"use strict";e.r(n);var r=e(2);const o=document.getElementById("canvas"),i=document.getElementById("fps"),u=r.c.new(),c=r.b.new(o);var f=0,a=0,s=!1;for(let t=0;t<100;t++)u.add_particle(r.a.new(100*Math.random(),100*Math.random(),.75+.25*Math.random()));let l=null;function p(t){l||(l=t);let n=(t-l)/1e3;i.innerHTML=`FPS: ${(1/n).toFixed(2)}, ${(1/60/n).toFixed(2)}x Real Time`,Object(r.R)(c,u,1/60),s&&u.apply_mouse_force(100*f,100*a,n),l=t,window.requestAnimationFrame(p)}function d(t,n){const e=t.getBoundingClientRect();return{x:(n.clientX-e.left)/e.width,y:(e.height-(n.clientY-e.top))/e.height}}o.addEventListener("mousemove",(function(t){const n=d(canvas,t);f=n.x,a=n.y})),document.addEventListener("mousedown",(function(t){s=!0;const n=d(canvas,t);f=n.x,a=n.y})),document.addEventListener("mouseup",(function(t){s=!1})),o.addEventListener("touchmove",(function(t){t.preventDefault();const n=d(o,t.changedTouches[0]);f=n.x,a=n.y})),o.addEventListener("touchstart",(function(t){t.preventDefault();const n=d(o,t.changedTouches[0]);f=n.x,a=n.y,s=!0})),o.addEventListener("touchend",(function(t){t.preventDefault(),s=!1})),Object(r.R)(c,u,0),setTimeout(()=>{window.requestAnimationFrame(p)},1e3)},function(t,n,e){"use strict";e.d(n,"R",(function(){return w})),e.d(n,"a",(function(){return j})),e.d(n,"b",(function(){return T})),e.d(n,"c",(function(){return _})),e.d(n,"B",(function(){return S})),e.d(n,"E",(function(){return P})),e.d(n,"r",(function(){return A})),e.d(n,"P",(function(){return D})),e.d(n,"N",(function(){return z})),e.d(n,"O",(function(){return L})),e.d(n,"h",(function(){return k})),e.d(n,"C",(function(){return F})),e.d(n,"g",(function(){return I})),e.d(n,"s",(function(){return $})),e.d(n,"q",(function(){return N})),e.d(n,"L",(function(){return R})),e.d(n,"y",(function(){return B})),e.d(n,"J",(function(){return C})),e.d(n,"H",(function(){return J})),e.d(n,"F",(function(){return M})),e.d(n,"I",(function(){return U})),e.d(n,"i",(function(){return H})),e.d(n,"j",(function(){return G})),e.d(n,"p",(function(){return q})),e.d(n,"t",(function(){return Z})),e.d(n,"z",(function(){return K})),e.d(n,"m",(function(){return Q})),e.d(n,"d",(function(){return V})),e.d(n,"A",(function(){return W})),e.d(n,"v",(function(){return X})),e.d(n,"M",(function(){return Y})),e.d(n,"u",(function(){return tt})),e.d(n,"K",(function(){return nt})),e.d(n,"l",(function(){return et})),e.d(n,"e",(function(){return rt})),e.d(n,"o",(function(){return ot})),e.d(n,"f",(function(){return it})),e.d(n,"G",(function(){return ut})),e.d(n,"n",(function(){return ct})),e.d(n,"D",(function(){return ft})),e.d(n,"k",(function(){return at})),e.d(n,"x",(function(){return st})),e.d(n,"w",(function(){return lt})),e.d(n,"Q",(function(){return pt}));var r=e(4);const o=new Array(32).fill(void 0);function i(t){return o[t]}o.push(void 0,null,!0,!1);let u=o.length;function c(t){const n=i(t);return function(t){t<36||(o[t]=u,u=t)}(t),n}let f=0,a=null;function s(){return null!==a&&a.buffer===r.u.buffer||(a=new Uint8Array(r.u.buffer)),a}let l=new("undefined"==typeof TextEncoder?e(3).TextEncoder:TextEncoder)("utf-8");const p="function"==typeof l.encodeInto?function(t,n){return l.encodeInto(t,n)}:function(t,n){const e=l.encode(t);return n.set(e),{read:t.length,written:e.length}};function d(t,n,e){if(void 0===e){const e=l.encode(t),r=n(e.length);return s().subarray(r,r+e.length).set(e),f=e.length,r}let r=t.length,o=n(r);const i=s();let u=0;for(;u<r;u++){const n=t.charCodeAt(u);if(n>127)break;i[o+u]=n}if(u!==r){0!==u&&(t=t.slice(u)),o=e(o,r,r=u+3*t.length);const n=s().subarray(o+u,o+r);u+=p(t,n).written}return f=u,o}let y=null;function h(){return null!==y&&y.buffer===r.u.buffer||(y=new Int32Array(r.u.buffer)),y}function g(t){u===o.length&&o.push(o.length+1);const n=u;return u=o[n],o[n]=t,n}let m=new("undefined"==typeof TextDecoder?e(3).TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});function b(t,n){return m.decode(s().subarray(t,t+n))}function v(t,n){if(!(t instanceof n))throw new Error("expected instance of "+n.name);return t.ptr}function w(t,n,e){v(t,T),v(n,_),r.x(t.ptr,n.ptr,e)}function O(t){r.q(g(t))}function x(t){return null==t}m.decode();class j{static __wrap(t){const n=Object.create(j.prototype);return n.ptr=t,n}free(){const t=this.ptr;this.ptr=0,r.g(t)}get pos(){var t=r.c(this.ptr);return E.__wrap(t)}set pos(t){v(t,E);var n=t.ptr;t.ptr=0,r.k(this.ptr,n)}get vel(){var t=r.d(this.ptr);return E.__wrap(t)}set vel(t){v(t,E);var n=t.ptr;t.ptr=0,r.l(this.ptr,n)}get mass(){return r.b(this.ptr)}set mass(t){r.j(this.ptr,t)}get force_acc(){var t=r.a(this.ptr);return E.__wrap(t)}set force_acc(t){v(t,E);var n=t.ptr;t.ptr=0,r.i(this.ptr,n)}static new(t,n,e){var o=r.v(t,n,e);return j.__wrap(o)}}class T{static __wrap(t){const n=Object.create(T.prototype);return n.ptr=t,n}free(){const t=this.ptr;this.ptr=0,r.h(t)}static new(t){var n=r.w(g(t));return T.__wrap(n)}}class E{static __wrap(t){const n=Object.create(E.prototype);return n.ptr=t,n}free(){const t=this.ptr;this.ptr=0,r.o(t)}get x(){return r.e(this.ptr)}set x(t){r.m(this.ptr,t)}get y(){return r.f(this.ptr)}set y(t){r.n(this.ptr,t)}static new(t,n){var e=r.z(t,n);return E.__wrap(e)}length(){return r.y(this.ptr)}}class _{static __wrap(t){const n=Object.create(_.prototype);return n.ptr=t,n}free(){const t=this.ptr;this.ptr=0,r.p(t)}static new(){var t=r.C();return _.__wrap(t)}add_particle(t){v(t,j);var n=t.ptr;t.ptr=0,r.A(this.ptr,n)}apply_mouse_force(t,n,e){r.B(this.ptr,t,n,e)}}const S=function(){return g(new Error)},P=function(t,n){var e=d(i(n).stack,r.s,r.t),o=f;h()[t/4+1]=o,h()[t/4+0]=e},A=function(t,n){try{console.error(b(t,n))}finally{r.r(t,n)}},D=function(t){c(t)},z=function(t,n){var e=d(function t(n){const e=typeof n;if("number"==e||"boolean"==e||null==n)return""+n;if("string"==e)return`"${n}"`;if("symbol"==e){const t=n.description;return null==t?"Symbol":`Symbol(${t})`}if("function"==e){const t=n.name;return"string"==typeof t&&t.length>0?`Function(${t})`:"Function"}if(Array.isArray(n)){const e=n.length;let r="[";e>0&&(r+=t(n[0]));for(let o=1;o<e;o++)r+=", "+t(n[o]);return r+="]",r}const r=/\[object ([^\]]+)\]/.exec(toString.call(n));let o;if(!(r.length>1))return toString.call(n);if(o=r[1],"Object"==o)try{return"Object("+JSON.stringify(n)+")"}catch(t){return"Object"}return n instanceof Error?`${n.name}: ${n.message}\n${n.stack}`:o}(i(n)),r.s,r.t),o=f;h()[t/4+1]=o,h()[t/4+0]=e},L=function(){return g(r.u)},k=function(t){return g(i(t).buffer)},F=function(t,n,e){return g(new Float32Array(i(t),n>>>0,e>>>0))},I=function(t,n,e,r){i(t).bufferData(n>>>0,i(e),r>>>0)},$=function(t,n,e,r){return i(t).getAttribLocation(i(n),b(e,r))},N=function(t,n){i(t).enableVertexAttribArray(n>>>0)},R=function(t,n,e,r,o,u,c){i(t).vertexAttribPointer(n>>>0,e,r>>>0,0!==o,u,c)},B=function(t,n,e,r){var o=i(t).getUniformLocation(i(n),b(e,r));return x(o)?0:g(o)},C=function(t,n,e,r){i(t).uniform2f(i(n),e,r)},J=function(t,n,e){i(t).uniform1f(i(n),e)},M=function(t,n,e,r,o,u,c,f,a,s,l){try{i(t).texImage2D(n>>>0,e,r,o,u,c,f>>>0,a>>>0,i(s),l>>>0)}catch(t){O(t)}},U=function(t,n,e){i(t).uniform1i(i(n),e)},H=function(t,n,e,r,o){i(t).clearColor(n,e,r,o)},G=function(t,n){i(t).clear(n>>>0)},q=function(t,n,e,r){i(t).drawArrays(n>>>0,e,r)},Z=function(t,n,e){try{var r=i(t).getContext(b(n,e));return x(r)?0:g(r)}catch(t){O(t)}},K=function(t){return i(t)instanceof WebGL2RenderingContext},Q=function(t){var n=i(t).createProgram();return x(n)?0:g(n)},V=function(t,n,e){i(t).attachShader(i(n),i(e))},W=function(t,n){i(t).linkProgram(i(n))},X=function(t,n,e){return g(i(t).getProgramParameter(i(n),e>>>0))},Y=function(t){const n=i(t);return"boolean"==typeof n?n?1:0:2},tt=function(t,n,e){var o=i(n).getProgramInfoLog(i(e)),u=x(o)?0:d(o,r.s,r.t),c=f;h()[t/4+1]=c,h()[t/4+0]=u},nt=function(t,n){i(t).useProgram(i(n))},et=function(t){var n=i(t).createBuffer();return x(n)?0:g(n)},rt=function(t,n,e){i(t).bindBuffer(n>>>0,i(e))},ot=function(t){var n=i(t).createTexture();return x(n)?0:g(n)},it=function(t,n,e){i(t).bindTexture(n>>>0,i(e))},ut=function(t,n,e,r){i(t).texParameteri(n>>>0,e>>>0,r)},ct=function(t,n){var e=i(t).createShader(n>>>0);return x(e)?0:g(e)},ft=function(t,n,e,r){i(t).shaderSource(i(n),b(e,r))},at=function(t,n){i(t).compileShader(i(n))},st=function(t,n,e){return g(i(t).getShaderParameter(i(n),e>>>0))},lt=function(t,n,e){var o=i(n).getShaderInfoLog(i(e)),u=x(o)?0:d(o,r.s,r.t),c=f;h()[t/4+1]=c,h()[t/4+0]=u},pt=function(t,n){throw new Error(b(t,n))}},function(t,n,e){(function(t){var r=Object.getOwnPropertyDescriptors||function(t){for(var n=Object.keys(t),e={},r=0;r<n.length;r++)e[n[r]]=Object.getOwnPropertyDescriptor(t,n[r]);return e},o=/%[sdj%]/g;n.format=function(t){if(!m(t)){for(var n=[],e=0;e<arguments.length;e++)n.push(c(arguments[e]));return n.join(" ")}e=1;for(var r=arguments,i=r.length,u=String(t).replace(o,(function(t){if("%%"===t)return"%";if(e>=i)return t;switch(t){case"%s":return String(r[e++]);case"%d":return Number(r[e++]);case"%j":try{return JSON.stringify(r[e++])}catch(t){return"[Circular]"}default:return t}})),f=r[e];e<i;f=r[++e])h(f)||!w(f)?u+=" "+f:u+=" "+c(f);return u},n.deprecate=function(e,r){if(void 0!==t&&!0===t.noDeprecation)return e;if(void 0===t)return function(){return n.deprecate(e,r).apply(this,arguments)};var o=!1;return function(){if(!o){if(t.throwDeprecation)throw new Error(r);t.traceDeprecation?console.trace(r):console.error(r),o=!0}return e.apply(this,arguments)}};var i,u={};function c(t,e){var r={seen:[],stylize:a};return arguments.length>=3&&(r.depth=arguments[2]),arguments.length>=4&&(r.colors=arguments[3]),y(e)?r.showHidden=e:e&&n._extend(r,e),b(r.showHidden)&&(r.showHidden=!1),b(r.depth)&&(r.depth=2),b(r.colors)&&(r.colors=!1),b(r.customInspect)&&(r.customInspect=!0),r.colors&&(r.stylize=f),s(r,t,r.depth)}function f(t,n){var e=c.styles[n];return e?"["+c.colors[e][0]+"m"+t+"["+c.colors[e][1]+"m":t}function a(t,n){return t}function s(t,e,r){if(t.customInspect&&e&&j(e.inspect)&&e.inspect!==n.inspect&&(!e.constructor||e.constructor.prototype!==e)){var o=e.inspect(r,t);return m(o)||(o=s(t,o,r)),o}var i=function(t,n){if(b(n))return t.stylize("undefined","undefined");if(m(n)){var e="'"+JSON.stringify(n).replace(/^"|"$/g,"").replace(/'/g,"\\'").replace(/\\"/g,'"')+"'";return t.stylize(e,"string")}if(g(n))return t.stylize(""+n,"number");if(y(n))return t.stylize(""+n,"boolean");if(h(n))return t.stylize("null","null")}(t,e);if(i)return i;var u=Object.keys(e),c=function(t){var n={};return t.forEach((function(t,e){n[t]=!0})),n}(u);if(t.showHidden&&(u=Object.getOwnPropertyNames(e)),x(e)&&(u.indexOf("message")>=0||u.indexOf("description")>=0))return l(e);if(0===u.length){if(j(e)){var f=e.name?": "+e.name:"";return t.stylize("[Function"+f+"]","special")}if(v(e))return t.stylize(RegExp.prototype.toString.call(e),"regexp");if(O(e))return t.stylize(Date.prototype.toString.call(e),"date");if(x(e))return l(e)}var a,w="",T=!1,E=["{","}"];(d(e)&&(T=!0,E=["[","]"]),j(e))&&(w=" [Function"+(e.name?": "+e.name:"")+"]");return v(e)&&(w=" "+RegExp.prototype.toString.call(e)),O(e)&&(w=" "+Date.prototype.toUTCString.call(e)),x(e)&&(w=" "+l(e)),0!==u.length||T&&0!=e.length?r<0?v(e)?t.stylize(RegExp.prototype.toString.call(e),"regexp"):t.stylize("[Object]","special"):(t.seen.push(e),a=T?function(t,n,e,r,o){for(var i=[],u=0,c=n.length;u<c;++u)P(n,String(u))?i.push(p(t,n,e,r,String(u),!0)):i.push("");return o.forEach((function(o){o.match(/^\d+$/)||i.push(p(t,n,e,r,o,!0))})),i}(t,e,r,c,u):u.map((function(n){return p(t,e,r,c,n,T)})),t.seen.pop(),function(t,n,e){if(t.reduce((function(t,n){return n.indexOf("\n")>=0&&0,t+n.replace(/\u001b\[\d\d?m/g,"").length+1}),0)>60)return e[0]+(""===n?"":n+"\n ")+" "+t.join(",\n  ")+" "+e[1];return e[0]+n+" "+t.join(", ")+" "+e[1]}(a,w,E)):E[0]+w+E[1]}function l(t){return"["+Error.prototype.toString.call(t)+"]"}function p(t,n,e,r,o,i){var u,c,f;if((f=Object.getOwnPropertyDescriptor(n,o)||{value:n[o]}).get?c=f.set?t.stylize("[Getter/Setter]","special"):t.stylize("[Getter]","special"):f.set&&(c=t.stylize("[Setter]","special")),P(r,o)||(u="["+o+"]"),c||(t.seen.indexOf(f.value)<0?(c=h(e)?s(t,f.value,null):s(t,f.value,e-1)).indexOf("\n")>-1&&(c=i?c.split("\n").map((function(t){return"  "+t})).join("\n").substr(2):"\n"+c.split("\n").map((function(t){return"   "+t})).join("\n")):c=t.stylize("[Circular]","special")),b(u)){if(i&&o.match(/^\d+$/))return c;(u=JSON.stringify(""+o)).match(/^"([a-zA-Z_][a-zA-Z_0-9]*)"$/)?(u=u.substr(1,u.length-2),u=t.stylize(u,"name")):(u=u.replace(/'/g,"\\'").replace(/\\"/g,'"').replace(/(^"|"$)/g,"'"),u=t.stylize(u,"string"))}return u+": "+c}function d(t){return Array.isArray(t)}function y(t){return"boolean"==typeof t}function h(t){return null===t}function g(t){return"number"==typeof t}function m(t){return"string"==typeof t}function b(t){return void 0===t}function v(t){return w(t)&&"[object RegExp]"===T(t)}function w(t){return"object"==typeof t&&null!==t}function O(t){return w(t)&&"[object Date]"===T(t)}function x(t){return w(t)&&("[object Error]"===T(t)||t instanceof Error)}function j(t){return"function"==typeof t}function T(t){return Object.prototype.toString.call(t)}function E(t){return t<10?"0"+t.toString(10):t.toString(10)}n.debuglog=function(e){if(b(i)&&(i=t.env.NODE_DEBUG||""),e=e.toUpperCase(),!u[e])if(new RegExp("\\b"+e+"\\b","i").test(i)){var r=t.pid;u[e]=function(){var t=n.format.apply(n,arguments);console.error("%s %d: %s",e,r,t)}}else u[e]=function(){};return u[e]},n.inspect=c,c.colors={bold:[1,22],italic:[3,23],underline:[4,24],inverse:[7,27],white:[37,39],grey:[90,39],black:[30,39],blue:[34,39],cyan:[36,39],green:[32,39],magenta:[35,39],red:[31,39],yellow:[33,39]},c.styles={special:"cyan",number:"yellow",boolean:"yellow",undefined:"grey",null:"bold",string:"green",date:"magenta",regexp:"red"},n.isArray=d,n.isBoolean=y,n.isNull=h,n.isNullOrUndefined=function(t){return null==t},n.isNumber=g,n.isString=m,n.isSymbol=function(t){return"symbol"==typeof t},n.isUndefined=b,n.isRegExp=v,n.isObject=w,n.isDate=O,n.isError=x,n.isFunction=j,n.isPrimitive=function(t){return null===t||"boolean"==typeof t||"number"==typeof t||"string"==typeof t||"symbol"==typeof t||void 0===t},n.isBuffer=e(6);var _=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];function S(){var t=new Date,n=[E(t.getHours()),E(t.getMinutes()),E(t.getSeconds())].join(":");return[t.getDate(),_[t.getMonth()],n].join(" ")}function P(t,n){return Object.prototype.hasOwnProperty.call(t,n)}n.log=function(){console.log("%s - %s",S(),n.format.apply(n,arguments))},n.inherits=e(7),n._extend=function(t,n){if(!n||!w(n))return t;for(var e=Object.keys(n),r=e.length;r--;)t[e[r]]=n[e[r]];return t};var A="undefined"!=typeof Symbol?Symbol("util.promisify.custom"):void 0;function D(t,n){if(!t){var e=new Error("Promise was rejected with a falsy value");e.reason=t,t=e}return n(t)}n.promisify=function(t){if("function"!=typeof t)throw new TypeError('The "original" argument must be of type Function');if(A&&t[A]){var n;if("function"!=typeof(n=t[A]))throw new TypeError('The "util.promisify.custom" argument must be of type Function');return Object.defineProperty(n,A,{value:n,enumerable:!1,writable:!1,configurable:!0}),n}function n(){for(var n,e,r=new Promise((function(t,r){n=t,e=r})),o=[],i=0;i<arguments.length;i++)o.push(arguments[i]);o.push((function(t,r){t?e(t):n(r)}));try{t.apply(this,o)}catch(t){e(t)}return r}return Object.setPrototypeOf(n,Object.getPrototypeOf(t)),A&&Object.defineProperty(n,A,{value:n,enumerable:!1,writable:!1,configurable:!0}),Object.defineProperties(n,r(t))},n.promisify.custom=A,n.callbackify=function(n){if("function"!=typeof n)throw new TypeError('The "original" argument must be of type Function');function e(){for(var e=[],r=0;r<arguments.length;r++)e.push(arguments[r]);var o=e.pop();if("function"!=typeof o)throw new TypeError("The last argument must be of type Function");var i=this,u=function(){return o.apply(i,arguments)};n.apply(this,e).then((function(n){t.nextTick(u,null,n)}),(function(n){t.nextTick(D,n,u)}))}return Object.setPrototypeOf(e,Object.getPrototypeOf(n)),Object.defineProperties(e,r(n)),e}}).call(this,e(5))},function(t,n,e){"use strict";var r=e.w[t.i];t.exports=r;e(2);r.D()},function(t,n){var e,r,o=t.exports={};function i(){throw new Error("setTimeout has not been defined")}function u(){throw new Error("clearTimeout has not been defined")}function c(t){if(e===setTimeout)return setTimeout(t,0);if((e===i||!e)&&setTimeout)return e=setTimeout,setTimeout(t,0);try{return e(t,0)}catch(n){try{return e.call(null,t,0)}catch(n){return e.call(this,t,0)}}}!function(){try{e="function"==typeof setTimeout?setTimeout:i}catch(t){e=i}try{r="function"==typeof clearTimeout?clearTimeout:u}catch(t){r=u}}();var f,a=[],s=!1,l=-1;function p(){s&&f&&(s=!1,f.length?a=f.concat(a):l=-1,a.length&&d())}function d(){if(!s){var t=c(p);s=!0;for(var n=a.length;n;){for(f=a,a=[];++l<n;)f&&f[l].run();l=-1,n=a.length}f=null,s=!1,function(t){if(r===clearTimeout)return clearTimeout(t);if((r===u||!r)&&clearTimeout)return r=clearTimeout,clearTimeout(t);try{r(t)}catch(n){try{return r.call(null,t)}catch(n){return r.call(this,t)}}}(t)}}function y(t,n){this.fun=t,this.array=n}function h(){}o.nextTick=function(t){var n=new Array(arguments.length-1);if(arguments.length>1)for(var e=1;e<arguments.length;e++)n[e-1]=arguments[e];a.push(new y(t,n)),1!==a.length||s||c(d)},y.prototype.run=function(){this.fun.apply(null,this.array)},o.title="browser",o.browser=!0,o.env={},o.argv=[],o.version="",o.versions={},o.on=h,o.addListener=h,o.once=h,o.off=h,o.removeListener=h,o.removeAllListeners=h,o.emit=h,o.prependListener=h,o.prependOnceListener=h,o.listeners=function(t){return[]},o.binding=function(t){throw new Error("process.binding is not supported")},o.cwd=function(){return"/"},o.chdir=function(t){throw new Error("process.chdir is not supported")},o.umask=function(){return 0}},function(t,n){t.exports=function(t){return t&&"object"==typeof t&&"function"==typeof t.copy&&"function"==typeof t.fill&&"function"==typeof t.readUInt8}},function(t,n){"function"==typeof Object.create?t.exports=function(t,n){t.super_=n,t.prototype=Object.create(n.prototype,{constructor:{value:t,enumerable:!1,writable:!0,configurable:!0}})}:t.exports=function(t,n){t.super_=n;var e=function(){};e.prototype=n.prototype,t.prototype=new e,t.prototype.constructor=t}}]]);