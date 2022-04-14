(function () {
  var q = {};
  var D = function (k, V, m) {
    m = this;
    try {
      (function (k, V, m, Z, U) {
        k.D = ((k.Rs = function (g, v, r) {
          (v = (r = function () {
            return g;
          }, function () {
            return r();
          }), v)[this.l] = function (Y) {
            g = Y;
          };
          return v;
        }, k.wy = (k.H = function (g, v, r, Y, M, A) {
          return v = (((Y = (M = (A = (r = function () {
            return r[(A.lM | 0) + (Y[A.gy] === g | 0) - !M[A.gy]];
          }, this), A.h), function () {
            return r();
          }), Y)[A.l] = function (F) {
            r[A.Z2] = F;
          }, Y)[A.l](v), Y);
        }, 0), k.Z = void 0, k).i = (k.kT = ut, (Z = 0, k.Qq = 0, k).R = (k.f7 = (k.s = false, k.cv = [], Gg), null), 0), void 0);
        k.Nx = 25;
        for (U = []; 128 > Z; Z++) {
          U[Z] = String.fromCharCode(Z);
        }
        (((k.J = ((Z = (((((((k.Pv = (((((k.n7 = ((((((k.G = (((u(k, 105, (k.F = k, k.Tw = (k.g = [], []), k.qx = function (g) {
          this.F = g;
        }, 0)), u)(k, 132, 0), u(k, 232, function (g, v, r) {
          r = (v = (r = (v = n(g), n(g)), g).T(v) != 0, g).T(r);
          if (v) {
            u(g, 105, r);
          }
        }), u)(k, 123, function (g, v) {
          v = g.T(n(g));
          nH(v, g);
        }), u(k, 131, function (g, v, r, Y) {
          (r = (Y = (Y = (r = n(g), n)(g), v = n(g), g).T(Y), g.T(r)), u)(g, v, r in Y | 0);
        }), false), u)(k, 182, function (g, v, r, Y) {
          r = (v = (Y = n(g), r = n(g), n(g)), Y = g.T(Y), g.T(r));
          u(g, v, +(Y == r));
        }), u(k, 50, function (g, v, r, Y) {
          Y = n(g);
          v = n(g);
          r = n(g);
          if (g.F == g) {
            v = g.T(v);
            r = g.T(r);
            g.T(Y)[v] = r;
            if (Y == 114) {
              g.o = void 0;
              if (v == 2) {
                B7(g);
              }
            }
          }
        }), u)(k, 214, k), u(k, 118, function (g, v, r, Y, M, A, F) {
          if ((F = (r = n(g), Ed(g)), v = "", g).g[101]) {
            M = g.T(101);
            A = M.length;
            for (Y = 0; F--;) {
              Y = ((Y | 0) + (Ed(g) | 0)) % A;
              v += U[M[Y]];
            }
          } else {
            while (F--) {
              v += U[n(g)];
            }
          }
          u(g, r, v);
        }), u(k, 150, function (g, v) {
          if (!B(false, g, true)) {
            v = hN(g);
            u(g, v.j, v.O.apply(v.L, v.U));
          }
        }), u(k, 240, 0), u)(k, 36, function (g, v, r, Y, M, A) {
          (2,4,3);
          if (!B(true, g, true)) {
            (2,3);
            // M is an array from the heap, v is a function from the heap (?)
            // If M is an object, M = Object.keys(M)
            if (Mm((g = (v = (r = (A = (v = (M = n(g), n(g)), n)(g), n(g)), M = g.T(M), r = g.T(r), g.T(v)), g.T(A)), M)) == "object") {
              for (Y in A = [], M) {
                A.push(Y);
              }
              M = A;
            }
            // Split M into g-sized slices (g is from heap) and run v on each chunk
            for (A = (g = (Y = 0, 0 < g ? g : 1), M.length); Y < A; Y += g) {
              v(M.slice(Y, (Y | 0) + (g | 0)), r);
            }
          }
        }), u(k, 237, function (g, v, r, Y) {
          (v = (r = (Y = (v = n(g), n)(g), n(g)), g.T(v)), Y = g.T(Y), u)(g, r, v[Y]);
        }), k).A_ = false, u(k, 91, function () {}), u)(k, 93, [0, 0, 0]), []), u(k, 112, function (g, v, r, Y) {
          v = (r = (r = n(g), Y = n(g), g.T(r)), g.T(Y));
          u(g, Y, v + r);
        }), u(k, 194, function (g, v, r) {
          (v = n(g), r = n(g), u)(g, r, "" + g.T(v));
        }), u(k, 215, k.W(4)), u(k, 219, function (g, v, r, Y, M, A) {
          if (!B(false, g, true)) {
            r = hN(g);
            M = r.U;
            Y = r.O;
            v = M.length;
            A = r.L;
            M = v == 0 ? A[Y]() : v == 1 ? A[Y](M[0]) : v == 2 ? A[Y](M[0], M[1]) : v == 3 ? A[Y](M[0], M[1], M[2]) : 2();
            u(g, r.j, M);
          }
        }), u)(k, 51, function (g) {
          Od(g, 1);
        }), u(k, 136, {}), u(k, 189, []), u(k, 205, function (g) {
          g.$T(3);
        }), u)(k, 67, function (g) {
          Od(g, 4);
        }), u)(k, 52, function (g, v, r, Y, M) {
          for (r = (Y = (M = (v = n(g), Ed(g)), 0), []); Y < M; Y++) {
            r.push(n(g));
          }
          u(g, v, r);
        }), u(k, 70, function (g, v) {
          (g = (v = n(g), g).T(v), W7)(g[0], g[2], g[1]);
        }), u(k, 161, []), u(k, 7, function (g, v, r, Y) {
          v = (r = (Y = n(g), n(g)), n(g));
          u(g, v, g.T(Y) || g.T(r));
        }), u(k, 162, function (g, v, r, Y, M) {
          if ((v = (Y = (r = (v = (M = (r = n(g), n(g)), Y = n(g), n(g)), g.T(r)), g).T(Y), M = g.T(M), g).T(v), r) !== 0) {
            v = Nm(Y, v, g, 1, r, M);
            G(r, M, v);
            u(g, 207, [r, M, v]);
          }
        }), u)(k, 180, 285), function (k) {
          for (k = 0; 64 > k; ++k) {
            z[k] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_".charAt(k);
            q["ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_".charAt(k)] = k;
          }
          q["/"] = (q[z[64] = "", "+"] = 62, 63);
          q["="] = 64;
        }(), function (g, v) {
          (v.push(g[0] << 24 | g[1] << 16 | g[2] << 8 | g[3]), v.push(g[4] << 24 | g[5] << 16 | g[6] << 8 | g[7]), v).push(g[8] << 24 | g[9] << 16 | g[10] << 8 | g[11]);
        }), u)(k, 61, I), u(k, 149, function (g, v, r, Y, M) {
          M = (Y = (r = (r = (M = (Y = (v = n(g), n(g)), n)(g), n(g)), g.T(r)), g.T(Y)), g.T(M));
          u(g, v, Nm(Y, M, g, r));
        }), u)(k, 25, function (g, v, r) {
          if (!B(false, g, true)) {
            v = n(g);
            r = n(g);
            u(g, r, function () {
              var Y = et(g.T(v));
              return eval(Y);
            }());
          }
        }), u(k, 153, function (g, v, r, Y) {
          if (r = g.n7.pop()) {
            for (Y = n(g); 0 < Y; Y--) {
              v = n(g);
              r[v] = g.g[v];
            }
            r[17] = (r[161] = g.g[161], g.g)[17];
            g.g = r;
          } else {
            u(g, 105, g.B);
          }
        }), u(k, 188, function (g, v, r) {
          r = (r = (v = (r = n(g), n)(g), g.T(r)), Mm(r));
          u(g, v, r);
        }), u(k, 145, function (g) {
          g.$T(4);
        }), u)(k, 119, function (g, v, r, Y, M, A, F, d, W, a, t, X, H) {
          for (H = (a = (d = ((t = (A = (X = n(g), v = 0), function (Q, C) {
            while (v < Q) {
              A |= n(g) << v;
              v += 8;
            }
            A >>= (C = A & (1 << Q) - 1, Q);
            v -= Q;
            return C;
          }), t(3)) | 0) + 1, t(5)), Y = [], M = 0); M < a; M++) {
            r = t(1);
            Y.push(r);
            H += r ? 0 : 1;
          }
          for (H = (M = ((H | 0) - 1).toString(2).length, F = [], 0); H < a; H++) {
            if (!Y[H]) {
              F[H] = t(M);
            }
          }
          for (t = 0; t < a; t++) {
            if (Y[t]) {
              F[t] = n(g);
            }
          }
          for (W = []; d--;) {
            W.push(g.T(n(g)));
          }
          u(g, X, function (Q, C, L, $J, x) {
            for ($J = (L = (C = [], 0), []); L < a; L++) {
                    (1,9,2);
              if (!Y[x = F[L], L]) {
                    (1,3,2);
                while (x >= C.length) {
                    (1,2);
                  C.push(n(Q));
                }
                x = C[x];
              }
              $J.push(x);
            }
            Q.D = (Q.Z = (L = Q.C, Q).H(L, W.slice()), Q.H(L, $J));
          });
        }), u)(k, 30, [160, 0, 0]), u(k, 48, function (g, v, r, Y) {
          v = (Y = (r = n(g), n)(g), n)(g);
          u(g, v, g.T(r) >>> Y);
        }), u(k, 207, 0), k.A = [], u)(k, 62, function (g, v, r, Y, M, A) {
          if (!B(false, g, true)) {
            Y = hN(g);
            A = Y.O;
            M = Y.L;
            v = Y.U;
            r = v.length;
            A = r == 0 ? new M[A] : r == 1 ? new M[A](v[0]) : r == 2 ? new M[A](v[0], v[1]) : r == 3 ? new M[A](v[0], v[1], v[2]) : r == 4 ? new M[A](v[0], v[1], v[2], v[3]) : 2();
            u(g, Y.j, A);
          }
        }), u(k, 186, function (g) {
          g.uM(4);
        }), u)(k, 17, 2048), window.performance) || {}, k).Is = Z.timeOrigin || (Z.timing || {}).navigationStart || 0, []), k).B = 0, E([XN], k), E)([h, V], k), E)([xJ, m], k);
        O(k, true, true);
      }(this, k, V));
    } catch (Z) {
      b(this, Z);
      V(function (U) {
        U(m.N);
      });
    }
  };
  var Z6 = function () {};
  var v7 = function (k, V, m, Z, U) {
    V = Z = 0;
    for (U = []; V < k.length; V++) {
      m = k.charCodeAt(V);
      if (128 > m) {
        U[Z++] = m;
      } else {
        if (2048 > m) {
          U[Z++] = m >> 6 | 192;
        } else {
          if ((m & 64512) == 55296 && V + 1 < k.length && (k.charCodeAt(V + 1) & 64512) == 56320) {
            m = 65536 + ((m & 1023) << 10) + (k.charCodeAt(++V) & 1023);
            U[Z++] = m >> 18 | 240;
            U[Z++] = m >> 12 & 63 | 128;
          } else {
            U[Z++] = m >> 12 | 224;
          }
          U[Z++] = m >> 6 & 63 | 128;
        }
        U[Z++] = m & 63 | 128;
      }
    }
    return U;
  };
  var rQ = function (k, V, m, Z) {
    if (Z = c[k.substring(0, 3) + "_"]) {
      return Z(k.substring(3), V, m);
    } else {
      return function (k, V) {
        k(function (m) {
          m(V);
        });
        return [function () {
          return V;
        }];
      }(V, k);
    }
  };
  var YJ = function (k, V, m, Z, U) {
    Z = rQ(k, (m = (U = function () {}, void 0), function (g) {
      if (U) {
        if (V) {
          f(V);
        }
        m = g;
        U();
        U = void 0;
      }
    }), !!V)[0];
    return {invoke: function (g, v, r, Y, M) {
      if (!v) {
        v = Z(r);
        if (g) {
          g(v);
        }
        return v;
      }
      Y = function () {
        m(function (A) {
          f(function () {
            g(A);
          });
        }, r);
      };
      if (m) {
        Y();
      } else {
        M = U;
        U = function () {
          (M(), f)(Y);
        };
      }
    }};
  };
  var qm = function (k) {
    return k;
  };
  var Mm = function (k, V, m) {
    if ((V = typeof k, V) == "object") {
      if (k) {
        if (k instanceof Array) {
          return "array";
        }
        if (k instanceof Object) {
          return V;
        }
        if ((m = Object.prototype.toString.call(k), "[object Window]") == m) {
          return "object";
        }
        if (m == "[object Array]" || typeof k.length == "number" && typeof k.splice != "undefined" && typeof k.propertyIsEnumerable != "undefined" && !k.propertyIsEnumerable("splice")) {
          return "array";
        }
        if (m == "[object Function]" || typeof k.call != "undefined" && typeof k.propertyIsEnumerable != "undefined" && !k.propertyIsEnumerable("call")) {
          return "function";
        }
      } else {
        return "null";
      }
    } else if (V == "function" && typeof k.call == "undefined") {
      return "object";
    }
    return V;
  };
  var jt = function (k, V) {
    return (V = typeof k, V == "object") && k != null || V == "function";
  };
  var z = {};
  var I = this || self;
  var AN = function (k, V) {
    if (!I.addEventListener || !Object.defineProperty) {
      return false;
    }
    V = (k = false, Object.defineProperty({}, "passive", {get: function () {
      k = true;
    }}));
    try {
      I.addEventListener("test", Z6, V);
      I.removeEventListener("test", Z6, V);
    } catch (m) {}
    return k;
  }();
  var bt = function (k, V) {
    (this.type = k, this).currentTarget = this.target = (this.defaultPrevented = false, V);
  };
  bt.prototype.preventDefault = function () {
    this.defaultPrevented = true;
  };
  bt.prototype.stopPropagation = function () {};
  var S = function (k, V, m, Z) {
    this.pointerId = (this.state = (this.metaKey = this.shiftKey = this.altKey = (this.charCode = (this.key = ((this.relatedTarget = this.currentTarget = (bt.call(this, k ? k.type : ""), this.target = null), this).button = this.screenY = this.screenX = this.clientY = this.clientX = this.offsetY = this.offsetX = 0, ""), this).keyCode = 0, this.ctrlKey = false), null), 0);
    this.pointerType = "";
    this.K = null;
    if (k) {
      Z = this.type = k.type;
      m = k.changedTouches && k.changedTouches.length ? k.changedTouches[0] : null;
      this.target = k.target || k.srcElement;
      this.currentTarget = V;
      V = k.relatedTarget;
      if (!V) {
        if (Z == "mouseover") {
          V = k.fromElement;
        } else if (Z == "mouseout") {
          V = k.toElement;
        }
      }
      this.relatedTarget = V;
      if (m) {
        this.clientX = m.clientX !== void 0 ? m.clientX : m.pageX;
        this.clientY = m.clientY !== void 0 ? m.clientY : m.pageY;
        this.screenX = m.screenX || 0;
        this.screenY = m.screenY || 0;
      } else {
        this.offsetX = k.offsetX !== void 0 ? k.offsetX : k.layerX;
        this.offsetY = k.offsetY !== void 0 ? k.offsetY : k.layerY;
        this.clientX = k.clientX !== void 0 ? k.clientX : k.pageX;
        this.clientY = k.clientY !== void 0 ? k.clientY : k.pageY;
        this.screenX = k.screenX || 0;
        this.screenY = k.screenY || 0;
      }
      this.button = k.button;
      this.keyCode = k.keyCode || 0;
      this.key = k.key || "";
      this.charCode = k.charCode || (Z == "keypress" ? k.keyCode : 0);
      this.ctrlKey = k.ctrlKey;
      this.altKey = k.altKey;
      this.shiftKey = k.shiftKey;
      this.metaKey = k.metaKey;
      this.pointerId = k.pointerId || 0;
      this.pointerType = typeof k.pointerType === "string" ? k.pointerType : QC[k.pointerType] || "";
      this.state = k.state;
      this.K = k;
      if (k.defaultPrevented) {
        S.S.preventDefault.call(this);
      }
    }
  };
  var QC = {2: "touch", 3: (function () {
    function m() {}
    (S.prototype = ((m.prototype = bt.prototype, S).S = bt.prototype, new m), S.prototype).constructor = S;
    S.vv = function (Z, U, g) {
      var v = Array(arguments.length - 2);
      for (var r = 2; r < arguments.length; r++) {
        v[r - 2] = arguments[r];
      }
      return bt.prototype[U].apply(Z, v);
    };
  }(), "pen"), 4: "mouse"};
  var it = "closure_listenable_" + (1e6 * (S.prototype.preventDefault = function (k) {
    if ((k = (S.S.preventDefault.call(this), this).K, k).preventDefault) {
      k.preventDefault();
    } else {
      k.returnValue = false;
    }
  }, S.prototype.stopPropagation = function () {
    S.S.stopPropagation.call(this);
    if (this.K.stopPropagation) {
      this.K.stopPropagation();
    } else {
      this.K.cancelBubble = true;
    }
  }, Math.random()) | 0);
  var R = function (k) {
    (this.src = (this.P = 0, k), this).$ = {};
  };
  var o0 = function (k) {
    k.M = (k.listener = null, (k.V = null, k.src = null, k).Y = true, null);
  };
  var FN = 0;
  var D6 = function (k, V, m, Z, U) {
    this.capture = (this.V = (this.type = U, (this.Y = this.v = false, this).listener = (this.src = (this.key = ++FN, this.M = V, k), Z), null), !!m);
  };
  ((R.prototype.add = function (k, V, m, Z, U, g, v) {
    if (-1 < (v = tN(U, ((k = (g = k.toString(), this.$[g]), k) || (k = this.$[g] = [], this.P++), k), V, Z), v)) {
      V = k[v];
      if (!m) {
        V.v = false;
      }
    } else {
      V = new D6(this.src, U, !!Z, V, g);
      V.v = m;
      k.push(V);
    }
    return V;
  }, R).prototype.remove = function (k, V, m, Z, U) {
    if (!((k = k.toString(), k) in this.$)) {
      return false;
    }
    if (-(V = tN(Z, (U = this.$[k], U), V, m), 1) < V) {
      o0(U[V]);
      Array.prototype.splice.call(U, V, 1);
      if (U.length == 0) {
        delete this.$[k];
        this.P--;
      }
      return true;
    } else {
      return false;
    }
  }, P = D.prototype, R).prototype.hasListener = function (k, V, m, Z, U) {
    return function (k, V, m) {
      for (m in k) {
        if (V.call(void 0, k[m], m, k)) {
          return true;
        }
      }
      return false;
    }((Z = (U = (m = V !== void 0, k) !== void 0) ? k.toString() : "", this).$, function (g, v) {
      for (v = 0; v < g.length; ++v) {
        if ((!U || g[v].type == Z) && (!m || g[v].capture == V)) {
          return true;
        }
      }
      return false;
    });
  };
  var tN = function (k, V, m, Z, U, g) {
    for (g = 0; g < V.length; ++g) {
      U = V[g];
      if (!U.Y && U.listener == m && U.capture == !!Z && U.M == k) {
        return g;
      }
    }
    return -1;
  };
  var H7 = "closure_lm_" + (1e6 * Math.random() | 0);
  var dQ = function (k, V, m, Z) {
    if (k.Y) {
      k = true;
    } else {
      V = new S(V, this);
      m = k.M || k.src;
      Z = k.listener;
      if (k.v) {
        c7(k);
      }
      k = Z.call(m, V);
    }
    return k;
  };
  var G = function (k, V, m, Z, U, g) {
    if (Z && Z.once) {
      a0(U, Z, m, k, V);
    } else if (Array.isArray(V)) {
      for (g = 0; g < V.length; g++) {
        G(k, V[g], m, Z, U);
      }
    } else {
      m = CH(m);
      if (k && k[it]) {
        k.ry(V, m, jt(Z) ? !!Z.capture : !!Z, U);
      } else {
        P7(m, U, Z, false, k, V);
      }
    }
  };
  var a0 = function (k, V, m, Z, U, g) {
    if (Array.isArray(U)) {
      for (g = 0; g < U.length; g++) {
        a0(k, V, m, Z, U[g]);
      }
    } else {
      m = CH(m);
      if (Z && Z[it]) {
        Z.C7(U, m, jt(V) ? !!V.capture : !!V, k);
      } else {
        P7(m, k, V, true, Z, U);
      }
    }
  };
  var I0 = function (k) {
    k = k[H7];
    if (k instanceof R) {
      return k;
    } else {
      return null;
    }
  };
  var W7 = function (k, V, m, Z, U, g) {
    if (Array.isArray(m)) {
      for (g = 0; g < m.length; g++) {
        W7(k, V, m[g], Z, U);
      }
    } else if (Z = jt(Z) ? !!Z.capture : !!Z, V = CH(V), k && k[it]) {
      k.OR(m, V, Z, U);
    } else if (k && (k = I0(k))) {
      m = k.$[m.toString()];
      k = -1;
      if (m) {
        k = tN(U, m, V, Z);
      }
      if (V = -1 < k ? m[k] : null) {
        c7(V);
      }
    }
  };
  var LH = function (k) {
    if (k in sd) {
      return sd[k];
    } else {
      return sd[k] = "on" + k;
    }
  };
  var St = 0;
  var sd = {};
  var P7 = function (k, V, m, Z, U, g, v, r) {
    if (!g) {
      throw Error("Invalid event type");
    }
    k = ((v = I0((r = jt(m) ? !!m.capture : !!m, U))) || (U[H7] = v = new R(U)), v.add(g, k, Z, r, V));
    if (!k.V) {
      if ((((V = function (k, V) {
        V = (k = dQ, function (m) {
          return k.call(V.src, V.listener, m);
        });
        return V;
      }(), k).V = V, V).src = U, V.listener = k, U).addEventListener) {
        if (!AN) {
          m = r;
        }
        if (m === void 0) {
          m = false;
        }
        U.addEventListener(g.toString(), V, m);
      } else if (U.attachEvent) {
        U.attachEvent(LH(g.toString()), V);
      } else if (U.addListener && U.removeListener) {
        U.addListener(V);
      } else {
        throw Error("addEventListener and attachEvent are unavailable.");
      }
      St++;
    }
  };
  var c7 = function (k, V, m, Z, U, g, v) {
    if (typeof k !== "number" && k && !k.Y) {
      if ((m = k.src) && m[it]) {
        m.Mx(k);
      } else if (U = k.V, v = k.type, m.removeEventListener ? m.removeEventListener(v, U, k.capture) : m.detachEvent ? m.detachEvent(LH(v), U) : m.addListener && m.removeListener && m.removeListener(U), St--, U = I0(m)) {
        v = k.type;
        if (v in U.$) {
          Z = U.$[v];
          b: if (typeof Z === "string") {
            V = typeof k !== "string" || k.length != 1 ? -1 : Z.indexOf(k, 0);
          } else {
            for (V = 0; V < Z.length; V++) {
              if (V in Z && Z[V] === k) {
                break b;
              }
            }
            V = -1;
          }
          if (V = (g = V, 0 <= g)) {
            Array.prototype.splice.call(Z, g, 1);
          }
          if (V) {
            o0(k);
            if (U.$[v].length == 0) {
              delete U.$[v];
              U.P--;
            }
          }
        }
        if (U.P == 0) {
          U.src = null;
          m[H7] = null;
        }
      } else {
        o0(k);
      }
    }
  };
  var R0 = "__closure_events_fn_" + (1e9 * Math.random() >>> 0);
  var p = function (k, V, m, Z, U, g) {
    if ((k = ((V = (((U = void 0, k && k[0] === N) && (U = k[2], V = k[1], k = void 0), g = m.T(161), 0) == g.length && (Z = m.T(132) >> 3, g.push(V, Z >> 8 & 255, Z & 255), U != void 0 && g.push(U & 255)), ""), k) && (k.message && (V += k.message), k.stack && (V += ":" + k.stack)), m).T(17), 3) < k) {
      (U = (V = v7((k -= (V = V.slice(0, (k | 0) - 3), (V.length | 0) + 3), V.replace(/\r\n/g, "\n"))), m.F), m).F = m;
      try {
        e(m, 215, l(V.length, 2).concat(V), 9);
      } finally {
        m.F = U;
      }
    }
    u(m, 17, k);
  };
  var u = function (k, V, m) {
    if (V == 105 || V == 132) {
      if (k.g[V]) {
        k.g[V][k.l](m);
      } else {
        k.g[V] = k.Rs(m);
      }
    } else if (V != 30 && V != 215 && V != 189 && V != 161 && V != 93 || !k.g[V]) {
      k.g[V] = k.H(k.T, m);
    }
    if (V == 114) {
      B7(k);
    }
  };
  var hN = function (k, V, m, Z, U, g) {
    Z = (g = ((U = {}, V = n(k), U).j = n(k), U.U = [], k.F == k ? (n(k) | 0) - 1 : 1), n(k));
    // Pull an array of values from the stack (each taken from the heap)
    // Define object props as follows: O = heap(pop), j = pop, U.length = pop, L = heap(pop), U = [heap(pop),heap(pop),...]
    for (m = 0; m < g; m++) {
      U.U.push(n(k));
    }
    for ((U.O = k.T(V), U).L = k.T(Z); g--;) {
      U.U[g] = k.T(U.U[g]);
    }
    return U;
  };
  var E = function (k, V) {
    V.A.splice(0, 0, k);
  };
  D.prototype.t_ = void 0;
  D.prototype.W = function (k, V) {
    for (V = []; k--;) {
      V.push(255 * Math.random() | 0);
    }
    return V;
  };
  var Ed = function (k, V) {
    if ((V = n(k), V) & 128) {
      V = V & 127 | n(k) << 7;
    }
    return V;
  };
  var Nm = function (k, V, m, Z, U, g, v) {
    return v = function () {
      if (m.F == m) {
        if (m.g) {
          var r = [lt, k, V, void 0, U, g, arguments];
          if (Z == 2) {
            E(r, m);
            var Y = O(m, false, false);
          } else if (Z == 1) {
            var M = !m.A.length;
            E(r, m);
            if (M) {
              O(m, false, false);
            }
          } else {
            Y = pH(m, r);
          }
          return Y;
        }
        if (U && g) {
          W7(U, v, g);
        }
      }
    };
  };
  var XN = [];
  var N = {};
  var pH = function (k, V, m, Z, U) {
    k.s = (U = V[0], false);
    if (U == J) {
      k.Nx = 25;
      k.h(V);
    } else if (U == T) {
      Z = V[1];
      try {
        m = k.N || k.h(V);
      } catch (g) {
        b(k, g);
        m = k.N;
      }
      Z(m);
    } else if (U == JN) {
      k.h(V);
    } else if (U == h) {
      k.h(V);
    } else if (U == xJ) {
      try {
        for (m = 0; m < k.Tw.length; m++) {
          try {
            Z = k.Tw[m];
            Z[0][Z[1]](Z[2]);
          } catch (g) {}
        }
      } catch (g) {}
      (k.Tw = [], V)[1](function (g, v) {
        k.Fw(g, true, v);
      }, function (g) {
        E([Tg], (g = !k.A.length, k));
        if (g) {
          O(k, true, false);
        }
      });
    } else {
      if (U == lt) {
        m = V[2];
        Z = V[6];
        u(k, 135, V[4] ? [Z[0].K] : Z);
        u(k, 136, m);
        return k.h(V);
      }
      if (U == Tg) {
        k.cv = [];
        k.g = null;
        k.J = [];
      } else if (U == XN && I.document.readyState === "loading") {
        k.R = function (g, v, r) {
          G(I.document, "DOMContentLoaded", (r = (v = false, function () {
            if (!v) {
              v = true;
              g();
            }
          }), r));
          G(I, "load", r);
        };
      }
    }
  };
  (D.prototype.Fw = function (k, V, m, Z, U) {
    m = Mm(m) === "array" ? m : [m];
    if (this.N) {
      k(this.N);
    } else {
      try {
        U = !this.A.length;
        Z = [];
        E([J, Z, m], this);
        E([T, k, Z], this);
        if (!V || !!U) {
          O(this, V, true);
        }
      } catch (g) {
        b(this, g);
        k(this.N);
      }
    }
  }, D).prototype.Gw = function (k, V, m, Z) {
    try {
      Z = k[((V | 0) + 2) % 3];
      k[V] = (k[V] | 0) - (k[((V | 0) + 1) % 3] | 0) - (Z | 0) ^ (V == 1 ? Z << m : Z >>> m);
    } catch (U) {
      throw U;
    }
  };
  var T = [];
  var l = function (k, V, m, Z) {
    for (Z = (m = (V | 0) - 1, []); 0 <= m; m--) {
      Z[(V | 0) - 1 - (m | 0)] = k >> 8 * m & 255;
    }
    return Z;
  };
  D.prototype.T = function (k, V) {
    V = this.g[k];
    if (V === void 0) {
      throw [N, 30, k];
    }
    return V();
  };
  var yC = [];
  var h = [];
  D.prototype.bM = function () {
    return n(this);
  };
  var b = function (k, V) {
    k.N = ((k.N ? k.N + "~" : "E:") + V.message + ":" + V.stack).slice(0, 2048);
  };
  var lt = [];
  var JN = [];
  D.prototype.C = (P.gy = "caller", function (k) {
    if (!(k = k().shift(), this).Z().length && !this.D().length) {
      this.D = this.Z = void 0;
    }
    return k;
  });
  P.Z2 = (D.prototype.YT = function (k, V, m) {
    if (k.length == 3) {
      for (m = 0; 3 > m; m++) {
        V[m] += k[m];
      }
      k = [13, 8, 13, 12, 16, 5, 3, 10, 15];
      for (m = 0; 9 > m; m++) {
        V[3](V, m % 3, k[m]);
      }
    }
  }, 36);
  var KH = function (k, V, m) {
    ((m = k.T(105), k.J && m < k.B) ? (u(k, 105, k.B), nH(V, k)) : u(k, 105, V), wQ(k), u)(k, 105, m);
    return k.T(136);
  };
  P.lM = 35;
  P.l = "toString";
  // 105 is the bit position in the stack
  // k.J is the position in the stack
  // y returns the given position and moves forward the bit position
  var y = function (k, V) {
    if (V >= k.B) {
      throw [N, 31];
    }
    u(k, 105, (V | 0) + 8);
    return k.J[V >> 3];
  };
  var J = [];
  var f = (D.prototype.Wv = void 0, I).requestIdleCallback ? function (k) {
    requestIdleCallback(function () {
      k();
    }, {timeout: 4});
  } : I.setImmediate ? function (k) {
    setImmediate(k);
  } : function (k) {
    setTimeout(k, 0);
  };
  var Tg = [];
  var c;
  var B7 = function (k) {
    k.xT = y(k, k.T(105)) << 24 | y(k, k.T(105)) << 16 | y(k, k.T(105)) << 8 | y(k, k.T(105));
    k.o = void 0;
  };
  var k9 = function (k, V, m, Z) {
    try {
      for (Z = 0; Z !== -1565581536;) {
        V = (V | 0) + (((k << 4 | 0) ^ k >>> 5) + (k | 0) ^ (Z | 0) + (m[Z & 3] | 0)) | 0;
        Z = Z + 3172301049 | 0;
        k = (k | 0) + (((V << 4 | 0) ^ V >>> 5) + (V | 0) ^ (Z | 0) + (m[Z >>> 11 & 3] | 0)) | 0;
      }
      return [V >>> 24, V >> 16 & 255, V >> 8 & 255, V & 255, k >>> 24, k >> 16 & 255, k >> 8 & 255, k & 255];
    } catch (U) {
      throw U;
    }
  };
  var xJ = [];
  P.Bv = false;
  var e = function (k, V, m, Z, U, g) {
    if (k.F == k) {
      U = k.T(V);
      if (V == 215) {
        V = function (v, r, Y, M) {
          if ((Y = (M = U.length, M | 0) - 4 >> 3, U.K7) != Y) {
            Y = [0, 0, g[(r = (Y << 3) - 4, U).K7 = Y, 1], g[2]];
            try {
              U.UR = k9(V0(U, (r | 0) + 4), V0(U, r), Y);
            } catch (A) {
              throw A;
            }
          }
          U.push(U.UR[M & 7] ^ v);
        };
        g = k.T(93);
      } else {
        V = function (v) {
          U.push(v);
        };
      }
      if (Z) {
        V(Z & 255);
      }
      k = 0;
      for (Z = m.length; k < Z; k++) {
        V(m[k]);
      }
    }
  };
  var P = D.prototype;
  var V0 = function (k, V) {
    return k[V] << 24 | k[(V | 0) + 1] << 16 | k[(V | 0) + 2] << 8 | k[(V | 0) + 3];
  };
  var n = function (k, V, m, Z) {
    if (k.Z) {
      return k.C(k.D);
    }
    return ((V = y(k, (m = (V = k.T(105), V >> 3), V)), k.o) != m >> 3 && (k.o = m >> 3, Z = k.T(114), k.h_ = k9(k.o, k.xT, [0, 0, Z[1], Z[2]])), V) ^ k.h_[m & k[T].length];
  };
  var nH = function (k, V) {
    V.n7.push(V.g.slice());
    V.g[105] = void 0;
    u(V, 105, k);
  };
  var CH = function (k) {
    if (typeof k === "function") {
      return k;
    }
    return k[k[R0] || (k[R0] = function (V) {
      return k.handleEvent(V);
    }), R0];
  };
  P.ER = (P.Vq = function () {
    return Math.floor(this.I());
  }, P.D2 = function (k, V, m) {
    V ^= V << 13;
    V ^= V >> 17;
    if (!(V = (V ^ V << 5) & m)) {
      V = 1;
    }
    return k ^ V;
  }, (P.Xw = function (k, V, m, Z) {
    while (m--) {
      if (m != 105 && m != 132 && V.g[m]) {
        V.g[m] = V[Z](V[k](m), this);
      }
    }
    V[k] = this;
  }, P).os = function () {
    return Math.floor(this.Qq + (this.I() - this.X));
  }, (P.I = (window.performance || {}).now ? function () {
    return this.Is + window.performance.now();
  } : function () {
    return +new Date;
  }, P).zw = function (k, V, m, Z, U, g) {
    for (m = (U = (g = [], 0), 0); m < k.length; m++) {
      U += V;
      for (Z = Z << V | k[m]; 7 < U;) {
        U -= 8;
        g.push(Z >> U & 255);
      }
    }
    return g;
  }, function (k, V, m, Z, U) {
    for (U = Z = 0; U < k.length; U++) {
      Z += k.charCodeAt(U);
      Z += Z << 10;
      Z ^= Z >> 6;
    }
    Z = new (k = (Z += Z << 3, Z ^= Z >> 11, Z + (Z << 15) >>> 0), Number)(k & (1 << V) - 1);
    Z[0] = (k >>> V) % m;
    return Z;
  });
  var wQ = function (k, V, m, Z, U, g) {
    if (!k.N) {
      k.i++;
      try {
        Z = (U = k.B, V = 0, 5001);
        for (m = void 0; (k.Bv || --Z) && (k.Z || (V = k.T(105)) < U);) {
          try {
            g = void 0;
            if (k.Z) {
              m = k.C(k.Z);
            } else {
              u(k, 132, V);
              g = n(k);
              m = k.T(g);
            }
            if (m && m.call) {
              m(k);
            } else {
              p([N, 21, g], 0, k);
            }
            k.s = true;
            B(false, k, false);
          } catch (v) {
            if (k.T(180)) {
              p(v, 22, k);
            } else {
              u(k, 180, v);
            }
          }
        }
        if (!Z) {
          p([N, 33], 0, k);
        }
      } catch (v) {
        try {
          p(v, 22, k);
        } catch (r) {
          b(k, r);
        }
      }
      k.i--;
    }
  };
  D.prototype.h = function (k, V) {
    V = (k = {}, {});
    return function (m, Z, U, g, v, r, Y, M, A, F, d, W, a, t, X, H, Q) {
      V = (Z = V, k);
      try {
        if ((d = m[0], d) == yC) {
          if (Z == k) {
            return 44;
          } else {
            return 87;
          }
        }
        if (d == h) {
          A = m[1];
          try {
            F = [];
            for (Y = 0; Y < A.length;) {
              t = (U = (v = (a = q[A.charAt(Y++)], Y < A.length ? q[A.charAt(Y)] : 0), ++Y, Y < A.length ? q[A.charAt(Y)] : 64), ++Y, Y < A.length ? q[A.charAt(Y)] : 64);
              ++Y;
              if (a == null || v == null || U == null || t == null) {
                throw Error();
              }
              if ((F.push(a << 2 | v >> 4), 64) != U) {
                F.push(v << 4 & 240 | U >> 2);
                if (t != 64) {
                  F.push(U << 6 & 192 | t);
                }
              }
            }
            u(this, 114, (this.B = (this.J = F, this.J.length << 3), [0, 0, 0]));
          } catch (C) {
            p(C, 17, this);
            return;
          }
          wQ(this);
        } else if (d == J) {
          m[1].push(this.T(215).length, this.T(17), this.T(189).length, this.T(30).length);
          u(this, 136, m[2]);
          if (this.g[238]) {
            KH(this, this.T(238));
          }
        } else {
          if (d == T) {
            (r = (F = m[2], l)((this.T(30).length | 0) + 2, 2), X = this.F, this).F = this;
            try {
              W = this.T(161);
              if (0 < W.length) {
                e(this, 30, l(W.length, 2).concat(W), 10);
              }
              e(this, 30, [1], 109);
              e(this, 30, l(this[T].length, 1));
              A = 0;
              A -= (this.T(30).length | 0) + 5;
              A += this.T(240) & 2047;
              H = this.T(215);
              if (4 < H.length) {
                A -= (H.length | 0) + 3;
              }
              if (0 < A) {
                e(this, 30, l(A, 2).concat(this.W(A)), 15);
              }
              if (4 < H.length) {
                e(this, 30, l(H.length, 2).concat(H), 156);
              }
            } finally {
              this.F = X;
            }
            (Q = this.W(2).concat(this.T(30)), Q)[1] = Q[0] ^ 6;
            Q[3] = Q[1] ^ r[0];
            Q[4] = Q[1] ^ r[1];
            if (M = this.eR(Q)) {
              M = "!" + M;
            } else {
              M = "";
              for (A = 0; A < Q.length; A++) {
                g = Q[A][this.l](16);
                if (g.length == 1) {
                  g = "0" + g;
                }
                M += g;
              }
            }
            (((Y = M, this).T(215).length = F.shift(), u)(this, 17, F.shift()), this.T(189).length = F.shift(), this.T(30)).length = F.shift();
            return Y;
          }
          if (d == JN) {
            KH(this, m[1]);
          } else if (d == lt) {
            return KH(this, m[1]);
          }
        }
      } finally {
        V = Z;
      }
    };
  }();
  var Od = function (k, V, m, Z) {
    (Z = (m = n(k), n(k)), e)(k, Z, l(k.T(m), V));
  };
  var O = function (k, V, m, Z, U, g) {
    if (k.A.length) {
      (k.G && 0(), k.A_ = V, k).G = true;
      try {
        U = k.I();
        k.X = U;
        k.J_ = U;
        g = function (k, V, m, Z) {
          while (V.A.length) {
            Z = (V.R = null, V).A.pop();
            try {
              m = pH(V, Z);
            } catch (U) {
              b(V, U);
            }
            if (k && V.R) {
              k = V.R;
              k(function () {
                O(V, true, true);
              });
              break;
            }
          }
          return m;
        }(V, k);
        Z = k.I() - k.X;
        k.Qq += Z;
        if (!(Z < (m ? 0 : 10)) && !(0 >= k.Nx--)) {
          Z = Math.floor(Z);
          k.cv.push(254 >= Z ? Z : 254);
        }
      } finally {
        k.G = false;
      }
      return g;
    }
  };
  D.prototype.eR = function (k, V, m, Z, U, g, v, r, Y, M) {
    for (r = (V = [], 0); r < k.length; r += 3) {
      U = r + 2 < k.length;
      Z = k[r];
      v = U ? k[r + 2] : 0;
      Y = r + 1 < k.length;
      M = Z >> 2;
      g = Y ? k[r + 1] : 0;
      m = (g & 15) << 2 | v >> 6;
      Z = (Z & 3) << 4 | g >> 4;
      v &= 63;
      if (!U) {
        v = 64;
        if (!Y) {
          m = 64;
        }
      }
      V.push(z[M], z[Z], z[m], z[v]);
    }
    return V.join("");
  };
  var B = function (k, V, m, Z) {
    Z = 0 < V.wy && V.G && V.A_ && 1 >= V.i && !V.Z && !V.R && (V.s || !m) && document.hidden == 0;
    if (!Z || (Z ? V.I() : V.J_) - V.X < V.wy - (k ? 255 : m ? 5 : 2)) {
      return false;
    }
    return !(V.R = ((k = V.T(m ? 132 : 105), u)(V, 105, V.B), V.A.push([JN, k]), f), 0);
  };
  D.prototype.uM = function (k, V, m) {
    for (m = (V = n(this), 0); 0 < k; k--) {
      m = m << 8 | n(this);
    }
    u(this, V, m);
  };
  D.prototype[xJ] = [0, 0, 1, 1, 0, 1, 1];
  var Gg = /./;
  D.prototype.$T = function (k, V, m, Z) {
    ((m = (Z = (V = k & 3, k &= 4, m = n(this), n(this)), this).T(m), k) && (m = v7(("" + m).replace(/\r\n/g, "\n"))), V && e(this, Z, l(m.length, 2)), e)(this, Z, m);
  };
  var ut = function (k) {
    return n(k) ^ n(k);
  };
  var et = (D.bind && (ut[D.prototype.l] = h.pop.bind(D.prototype[J]), Gg[D.prototype.l] = h.pop.bind(D.prototype[J])), function (k, V) {
    if ((V = function (k, V) {
      k = null;
      V = I.trustedTypes;
      if (!V || !V.createPolicy) {
        return k;
      }
      try {
        k = V.createPolicy("bg", {createHTML: qm, createScript: qm, createScriptURL: qm});
      } catch (m) {
        if (I.console) {
          I.console.error(m.message);
        }
      }
      return k;
    }()) && k.eval(V.createScript("1")) === 1) {
      return function (m) {
        return V.createScript(m);
      };
    } else {
      return function (m) {
        return "" + m;
      };
    }
  })(I);
  var w = I;
  var Z2 = ["botguard"];
  if (!("botguard" in w) && typeof w.execScript != "undefined") {
    w.execScript("var botguard");
  }
  for (var K; Z2.length && (K = Z2.shift());) {
    w = w[K] && w[K] !== Object.prototype[K] ? w[K] : w[K] = {};
  }
  (40 < (c = I.botguard, c.m) || (c.m = 41, c.bg = YJ, c.a = rQ), c).Xdf_ = function (k, V, m) {
    m = new D(k, V);
    return [function (Z) {
      return function (k, V, m) {
        V.Fw(function (Z) {
          m = Z;
        }, false, k);
        return m;
      }(Z, m);
    }];
  };
  try {
    if (!c.u) {
      G(I, "unload", function () {});
      c.u = 1;
    }
  } catch (k) {}
}.call(this));
