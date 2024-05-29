use std::collections::HashMap;

use serde::{Deserialize, Serialize};
// {
// 	"registers": {
// 		"34": "SET_OBJ",
// 		"66": "GET_OBJ",
// 		"74": "ARRAY_PUSH",
// 		"82": "SPLICE_POP",
// 		"85": "ARR_POP",
// 		"91": "JUMP_IF",
// 		"101": "THROW_ERROR",
// 		"114": "NEW_CLASS",
// 		"127": "NEW_ARR",
// 		"139": "SHUFFLE_REG",
// 		"146": "UNARY_EXP",
// 		"150": "BINARY_EXP",
// 		"178": "JUMP",
// 		"191": "BIND_FUNC",
// 		"200": "APPLY",
// 		"207": "NEW_OBJ",
// 		"228": "SET_MEM",
// 		"255": "LITERAL",
// 		"window": "40",
// 		"vmData": "8"
// 	},
// 	"magicBits": {
// 		"startEnc": 124,
// 		"opcode": {
// 			"a": 79,
// 			"b": 256
// 		},
// 		"encryption": {
// 			"a": 24397,
// 			"b": 61245
// 		},
// 		"NEW_ARR": [
// 			235
// 		],
// 		"JUMP_IF": [
// 			240,
// 			192
// 		],
// 		"UNARY_EXP": {
// 			"all": [
// 				92,
// 				28,
// 				43,
// 				104,
// 				20,
// 				44,
// 				67,
// 				176,
// 				83,
// 				169
// 			],
// 			"typeof": {
// 				"typeId": 98,
// 				"bits": [
// 					92,
// 					28
// 				]
// 			},
// 			"+": {
// 				"typeId": 50,
// 				"bits": [
// 					20,
// 					44
// 				]
// 			},
// 			"-": {
// 				"typeId": 182,
// 				"bits": [
// 					43,
// 					104
// 				]
// 			},
// 			"!": {
// 				"typeId": null,
// 				"bits": [
// 					null,
// 					null
// 				]
// 			},
// 			"~": {
// 				"typeId": 147,
// 				"bits": [
// 					83,
// 					169
// 				]
// 			}
// 		},
// 		"BIND_FUNC": [
// 			32,
// 			164
// 		],
// 		"ARRAY_PUSH": [
// 			5,
// 			147
// 		],
// 		"JUMP": [
// 			192
// 		],
// 		"NEW_OBJ": [
// 			95
// 		],
// 		"SET_MEM": [
// 			205,
// 			94
// 		],
// 		"APPLY": [
// 			227,
// 			238,
// 			27,
// 			187,
// 			13
// 		],
// 		"SET_OBJ": [
// 			133,
// 			159,
// 			78
// 		],
// 		"BINARY_EXP": {
// 			"all": [
// 				97,
// 				181,
// 				78,
// 				38,
// 				51,
// 				176,
// 				13,
// 				193,
// 				10,
// 				248,
// 				109,
// 				46,
// 				169,
// 				3,
// 				97,
// 				57,
// 				249,
// 				180,
// 				174,
// 				211,
// 				142,
// 				178,
// 				174,
// 				122,
// 				78,
// 				48,
// 				182,
// 				37,
// 				61,
// 				208,
// 				190,
// 				11,
// 				21,
// 				157,
// 				192,
// 				254,
// 				213,
// 				138,
// 				38,
// 				126,
// 				247,
// 				170,
// 				126,
// 				200,
// 				243,
// 				62,
// 				68,
// 				84,
// 				104,
// 				77,
// 				253,
// 				150,
// 				31
// 			],
// 			"+": {
// 				"typeId": 151,
// 				"enc": 97,
// 				"left": 181,
// 				"right": 78
// 			},
// 			"-": {
// 				"typeId": 52,
// 				"enc": 38,
// 				"left": 51,
// 				"right": 176
// 			},
// 			"*": {
// 				"typeId": 228,
// 				"enc": 13,
// 				"left": 193,
// 				"right": 10
// 			},
// 			"/": {
// 				"typeId": 251,
// 				"enc": 248,
// 				"left": 109,
// 				"right": 46
// 			},
// 			"%": {
// 				"typeId": 2,
// 				"enc": 169,
// 				"left": 3,
// 				"right": 97
// 			},
// 			"&&": {
// 				"typeId": 243,
// 				"enc": 57,
// 				"left": 249,
// 				"right": 180
// 			},
// 			"||": {
// 				"typeId": 240,
// 				"enc": 174,
// 				"left": 211,
// 				"right": 142
// 			},
// 			"^": {
// 				"typeId": 133,
// 				"enc": 37,
// 				"left": 61,
// 				"right": 208
// 			},
// 			"&": {
// 				"typeId": 6,
// 				"enc": 178,
// 				"left": 174,
// 				"right": 122
// 			},
// 			"|": {
// 				"typeId": 204,
// 				"enc": 78,
// 				"left": 48,
// 				"right": 182
// 			},
// 			"<<": {
// 				"typeId": 122,
// 				"enc": 190,
// 				"left": 11,
// 				"right": 21
// 			},
// 			">": {
// 				"typeId": 219,
// 				"enc": 62,
// 				"left": 68,
// 				"right": 84
// 			},
// 			">>": {
// 				"typeId": 150,
// 				"enc": 157,
// 				"left": 192,
// 				"right": 254
// 			},
// 			">>>": {
// 				"typeId": 108,
// 				"enc": 213,
// 				"left": 138,
// 				"right": 38
// 			},
// 			"==": {
// 				"typeId": 188,
// 				"enc": 126,
// 				"left": 247,
// 				"right": 170
// 			},
// 			"===": {
// 				"typeId": 174,
// 				"enc": 126,
// 				"left": 200,
// 				"right": 243
// 			},
// 			">=": {
// 				"typeId": null,
// 				"left": null,
// 				"right": null,
// 				"enc": null
// 			},
// 			"instanceof": {
// 				"typeId": 22,
// 				"enc": 253,
// 				"left": 150,
// 				"right": 31
// 			}
// 		},
// 		"GET_OBJ": [
// 			39,
// 			113,
// 			5
// 		],
// 		"SPLICE_POP": [
// 			58
// 		],
// 		"NEW_CLASS": [
// 			75,
// 			14,
// 			97,
// 			52
// 		],
// 		"THROW_ERROR": [
// 			248
// 		],
// 		"ARR_POP": [
// 			80,
// 			159
// 		],
// 		"SHUFFLE_REG": [
// 			173,
// 			203
// 		],
// 		"LITERAL": {
// 			"all": [
// 				63,
// 				253,
// 				31,
// 				182,
// 				192,
// 				164,
// 				250,
// 				228,
// 				213
// 			],
// 			"null": {
// 				"typeId": 45,
// 				"bits": []
// 			},
// 			"NaN": {
// 				"typeId": 15,
// 				"bits": []
// 			},
// 			"Infinity": {
// 				"typeId": 1,
// 				"bits": []
// 			},
// 			"true": {
// 				"typeId": 132,
// 				"bits": []
// 			},
// 			"false": {
// 				"typeId": 51,
// 				"bits": []
// 			},
// 			"number": {
// 				"typeId": 157,
// 				"bits": []
// 			},
// 			"bind": {
// 				"typeId": null,
// 				"bits": []
// 			},
// 			"bit": {
// 				"typeId": 247,
// 				"bits": []
// 			},
// 			"string": {
// 				"typeId": 136,
// 				"bits": [
// 					182
// 				]
// 			},
// 			"byte": {
// 				"typeId": null,
// 				"bits": []
// 			},
// 			"stack": {
// 				"typeId": 152,
// 				"bits": [
// 					192
// 				]
// 			},
// 			"regex": {
// 				"typeId": 107,
// 				"bits": [
// 					164,
// 					250,
// 					228
// 				]
// 			},
// 			"array": {
// 				"typeId": 191,
// 				"bits": [
// 					213
// 				]
// 			}
// 		}
// 	},
// 	"bytecodes": {
// 		"init": "EdiHXGCH4AsHLPWo3uUGYe3c41gM3SxZQ8SBQGqt4ikJgU8FeNGoVb++HTo2iw4H5UZLwiSQVBR75SlhQsZqQoGsNygQcFD0d2ph7o5x8vXd4I9kvJuYF7OVtRGa8+3D043AUNDrCeMfDlIMb0LrPAhTBbFCuu426xSRCFuWwchWZiAhB7HvVOCRHWnwkhTu424JoJxwWVvgnteCf9lFjASJVeQCihbpu7LOGKBQEXgg9lKf1yA4pCgzreGfYt9Bs9/DsMvV6RX9bicYT887nKeg5VzRGg44y75TaPBE6amm168rJgBx9jqCeCWxQMVU4kkAIEA6Yxv/FOUZH37YafAiWmA7U9kVKzqXtyWs6A==",
// 		"main": "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usPGm72l3LbCu+ndwLvtv+/Jyvf2zq7Rztvf39TbzvDu2tbc2wf33vztDAH7zQHdF8nq9foGGfruDwwa8PQFGPoHBCXiKxXfBe8DAyAdBxMOITD5HPEs/DsVNgAuMRoZJz81HBwDCR4jKykQTk81UBYXRRhZMC9YXkoeIFw4MDdnY0NVaFQqNkEta11wdEZUTEx1QnNvblk6aUprO0N1WGiJXWVdjHZlSWtuW0yIkVSAT4lTl5dmVFiMYFx7mW+Pf5h6eqyCjoirhG57c3+EpKl0pZW5kI6xiLR+obZ7ocKZpcbFoKPGhMm6u6StlMmS1Kypk5nbk7Wvvs270uWysLvoqba/66TuxeHF8cy+yebGy/e0sey1vtq40u3LBADa3tAKyNnI1gQB2OXe3OzP0NcXDO3m9/ny1fj58dkT8fHw9PkABBnsBAn4AeztMif29+70EAcvPRMaDTD6GSAiHCERIQYUJUgYGR1RJx0sDSUhSjYTMTUWKyBfOSMWLVA8QzVoQCI1YD1IXyo/P2NrKUJVSTNrcnNYXXdRXTxWe2NYcYhJfl9vgoVhjWRth01UknaWioeed2p2bHp4fpZlYH+qd3SAn5ithX6xnJSzdpZ4iY6bjbmLh3Kvqr+1pY6/xKObnKGpq42uwY2fk8aPr7XDtZmcyJzRqNOvnrXRtuTF36bq3OPH7qin3Omt5rLW8cjN6LT8AL7Z3OPb7N7z1NrT070JAgXv4RL87uTl0NTsC+LTGvHp3Qrd+fcfJPzgBekkBSwO/BzsCOwj6+c0KfY1FPbz9B72GfMXMQUEFQBIMxUIPg0uJlA7BQ1VFCAuLFU0KD4+NBtSQiFfF1ZgWztAWUY4V0Uia2xNM3BJMUNKNFVcWTp3XmlVTkE7dEVCSIV5YlZmSmBHam5OcXKEV2yEWGebbHGRmHSWgo5/XpqiZJOgbZl+nmp8s4eFbJCHdLGQrLh2i3iJi8OOjafDh4bAur+1ppmbrqquxcywzdCx1tW0j8ebmNPX1MK0nOTDs6DUuOXnq+njweW0v86t8uHit7rQx9v417kFwwMG2uPjAdnYCu3LyP3h5NEA9vHr0tru6u/x8tYR6w3eAN7gFRIr6gbsEC4ILgM0BTI29eosKwj8F/42NPoPGA8WJQREJxkEQi4dCEI6DC1GMjUxUDspLThGLRxcOGQ5O1M0OjdcJFxKJzgnRGxcRi93cTV4e2tUT1V7c1RjbF5UgYN6g0CFVkOOiIxPkGaRdISAcpmVjItrko+RoJeLf4NlnolnmqWIfY6FhGiQq3Bxkpd4rZati312iY6bscGDuqSlm5eloLqbl4vKz62QoMrL1smLu8PUs5WfuN7avM2cx52wwevTvranqvDLxt+v8uew6LD1x7j68Pzy9d7c3/C9+QHX9+b+0Q=="
// 	},
// 	"chlData": {
// 		"cvId": "3",
// 		"cZone": "cfschl.peet.ws",
// 		"cType": "managed",
// 		"cNounce": "20798",
// 		"cRay": "87861bb15bf7191b",
// 		"cHash": "49c3e13f69491a7",
// 		"cUPMDTk": "/?__cf_chl_tk=G5ZSPwN0O8Ts8hLH9eB_Bi4Y69gdJcD57Da.SJZrfXU-1713794091-0.0.1.1-1514",
// 		"cFPWv": "b",
// 		"cTTimeMs": "1000",
// 		"cMTimeMs": "375000",
// 		"cTplV": 5,
// 		"cTplB": "cf",
// 		"cK": "visitor-time",
// 		"fa": "/?__cf_chl_f_tk=G5ZSPwN0O8Ts8hLH9eB_Bi4Y69gdJcD57Da.SJZrfXU-1713794091-0.0.1.1-1514",
// 		"md": "ID9SUnwEI0DTE4KO471x8gNXDJGhbpPKbtzwRlEVKh4-1713794091-1.1.1.1-.xvqQSUf5Lp0cMq5.R3WjUOGNuQsD6XTzlKbg0ley7IlSjPNmUNSF2ECGtMlgcDadOitIhvsdMaNPoatxHMhDSmjZB6ynQrHgV1aWhyJT7xMg1TgKGCOrszrXkcgXq1mQ18_tSipyEX41UklvkQhQLo.uBFRfymnHze0iWpg0b72dx7bLsPi0gG1OpzC8gRlkQqPKTtNsW.vS7cDGauWu1j1r4ayNIYxNfOidfvRnPMBDP65HlcRJVdhuE5QF5YaQIEchG5Y4ZRn3RJCyIDiCsGMmD28yO7h3cTZEFnoEpSt2VqLTWeGGzn6AXQ.Ep8l69TQJ21MQYLTjl9wgKbP16Eh7ICKay3yGruYRIjl0qfJpTXg4x1DLaHm3MpPIMxQR.djQwuhS6N8NuKjN_klZPcarNSUsoHjyOuVs9ooQTPwq7UHOaxOq_K9.AdX3pPnieKyFMdEI41k1Qo1i6icX065pIzYNvqpwa9R3RzdAcyzt5HqmP3TlFcRR4uAMGgs6fYWsXWx0svrfu5eNd18iGoiSnSAZjpNUyDeugGeg.WN7xOzs.UoFMuHuGoaowvakbJcsO_nf_2GfC1cQ82nlMNYGCooR1qX0hUIi_HbMcsY8NQZ1XsSVcbSG.aEFsxrpW0i6C96o9.B4jtSEQG9RTmPpL1C7Nyb.pVgcdeFDCw33tGxNZjfs8Qeel4IYPmSCcpI.3SdJiOXrgPcI4kLLxZ8HPMSDLPdInT1emNwoSxTL00rYYienavM5NekKg5UFNqz4j663u9A5J1L_I2DLBMrXpITabvrgm.FyjJHm8nMIgVgED3Ml0JxN81KDZ_lHL58DW439yX8MJjUcq67SrZXF2bQzScpZ_vkFX.xzuE3z92kC7GiXyw8wBZ1EmUwNE_fawzti7AgRSkDkyur2Wl87nUFeUqpWiPJgOfolgecH2N7X40j.76zPquQGzA_UnDH8NCPhD6fYXM5PGmrsmKBG5zFgbR9sRhjZqcdfOg4dyOKcrO9EW4donNt5cTwR.FZUKHqQPhOGC1JlimIvrCvFSa2EdKIAi3xlvfhzM.CEaOAtdEVb.KQmfLW1yneqq5KJ6CuPIHeYNNn.m4it9nfpepaerkViki6VK169XR7CnfN4tVFIrB8xj6E66SrSDe78hlB23La1tjSvZfn_wmWfMlK3DSNjgXAxJ9L4c5YUz7YxdZc5RMdOyQEaBqfMre6Thl_bWia8BpkxStMFQPgSzhLCZ8sYwjF3UnyFFJtbRpc0J6hlThkOoJ77P9wfDE5kwe.O3MqrNz8nbV433jLP7DiKUuk9SD2t8fPsqkHscBXrDCf0iOMLuW1zYSfH40ivvc7sE4FlZpoSjTGger.YK_bKRynfn5a9Ootnxc9xddLDvls5RkaIJSDaqm3Qa_II.Gn32jWt0ZEenRExQFDcAh.oLswMZ7TP65CKEk",
// 		"mdrd": "VOQtZLeMD1sVuugEwGcA.DeA48DN3hyABELieBFXb7o-1713794091-1.1.1.1-ohpxpplsh45tVPUCA.ZM6sT4bfWOv0jku4xtgSuXZJLJ1ydTUUu.HtrLU3vhYzaDN7KttixPUa8.3GNZ8v_Y.G9kBmgZtPR_Qd5vQwcGwy8yGA7efhR90jHrgZEE8JvUTMd9PsdgXC33hc8l7rZzCU5vX46NZz.5gzckq1gKVOccbRbi7gXu2csCgqPZyC2YiJecziPQn8PNN5.pDNSOZPIuIFaBXexhl.X.zL3lw7CbWWMnZlzKMdhzK.0DT_4qUxogkWjdunhBCjFXNSzHvfhVNKwK8EDhKvttlEeAZrCFNV2ymKRnByVU9QGS24yjAFzCdbIDs9QkiCO3VBZ1UeYur7X_AQPIWGXnkysdOVSMHzInKJeL4hr76YaZ0z24mhlmBO5Gx8t6HYKKNH23K.omXb_X5xkpuY1w54c7DpjRtkxx4GX.L2YaK5oO85M1ahH9MeiAM4hPv53hbbKXVon12rMZDIdtgwNUEvHGk2o9sZnvjuRf9NAdcNRrX6jkCZN9l62l2h0_7i1JXuKUwu6BuUyBgIiYnKH2RRDfevv7s2AZTgF86FZshW8kXUXaA5aDXxNGKovzLdeF5TiLIkKzZBujOuH0u7PKMILlGX8iYWOWeSGcEHhYtPM81IjunOsNtTkx.XB4G7zxOO.CYxzHsIPZsiCIm7yl1.s0SW8MsDPesxfLorhI5yRKsWU_WHpuugmXDCXqRSnVJqbvTbAAG328Q0Pr088114SkvzV_QxYyQTt19HjCmtVZLo1M3ba.QBptrTMk0TDjCMqDIWELKosey_JPr_zwLoo1cZv..CvP2R0R9fkWuCElivJUvajliUAUJJyBOX31BDbFq80nKO0LuJUKfKHSyZ8oxpL7JuWKG1VBXKfnXF9XTBl_6nIqHn6WFoFkzKwyiaEQksFaedOTvIO8j0hlHusG0eOeSi9tnsxmBB.Cuf4Dh49r1kHPocQp9aE.n6YmcM8VE2iuWxlpCe7Ryg8_pU9tBrIzg.yxlQ9y0rkxAAYZkb7sKpXJlrlTnIZpRIBF1HYbw6kIfEJUzd9TEhIRrkZMMHeRcoQ4d60XNyWp06NtIvNY9b.ukX_iokUI6Kxk49PUJAsBIPq3ymcjROS9dHp9.oM0H0v2t19P7SXHI5jRmN5GvJ6F5HBjpXxnzAwPazOOKH02uAGq8FeInoWojI9Ovjctsx0t6uS_MKznVSEOgyZH0qiqJi1nHsuuhS9PEzFA6RDM0S0oKWpssVosnrXZfsOvPe7h9wxr0cw2gTS4L_WsDesdX4BGGvBJPpQxeQswbF5fKLOCTDjLxRiQLQ2tJhowTgbBuTHYs0IND055lEdhc9yGJZ_MoIiHnmlmUkhGVUbIrWPhy9QYJL1OZDZ1a_MW8QPWZIniN.JK_3x.fsjBsp1MW7DgxnO7SvQ8z4eaywsKdOoLrjiuJqAt6I_2Ne68s8NwIHbCuF2TiB3fc7v0qB1DWJYMEBekXuOiZ7ZF7l1cSMfqfaMJD1DB0bGoxsXqS.qmMp9U3WfUSYZFgq7j5h3Wq1H8nX5A98XPXTEJEu77sMTSFkUxUTS6D1nwTppryuXzfb.8TezhXqpb3x5zdLMAI7Sx5hjSYOmdNj_LnbEIDu8Y0EtyxLhpdmZiR9VlHM5tLcxfck4.7xMqefAAisRzskAmocNJAU8M6Oknd9nvi4auFtmPfWSsEqUK2S7h.ocKSRFoM96D6Q5vHSsodMPnijbVJOe58Kwm4N_hLR3Fo1fd1enB.BvzMKNgJl2JMuWiC3muY.DofdACHSSMXrs4.21a1x_HIpjRs4IG63ILdQaNbH6RBJqAELsaOXauPtlhGjq.sU93QNEg6MoxAz9ZX_c.tKFBCalZ_XAYbBw3tmFpe8f8tFe0BU3PxOROMGZ5ZUwf4Xg6NETtWvrzWFgAODXzCXhcS4NAWnrpbiPzV8cQyjYznqvezo9m7sUL6Uhy9z1h2MNhNC.4mTbVg.ONrCX1wU4I7MAp1G9B3moAqbZDDexmUduzodmEUt4G1D9oYMfOVhE2S_NkYVvXCmJjAv5sHSljY6cwFEYPry8DLwtU1XHz716F_KXMOJ.9neRzkvzc9Dsg9M_z2MyMuldYHEUjZjd0vp4iQ3TtYgIXW4uZ2.zwFU3C0cpCeiKRvV6sbQtkRYKC4_wIL6NVrAKD8WuJ6msgVosB.EzOp3j2UAPxbMFXSQkfWcFKfpitQd7dloypvT89vhWLoMDj9k.QrV3W9_bc21IWuSBH0Hx8nMbJfQTQZ87FplBeqhivMm.DLPu4ANWRSrIGBPt6UwjwFE7QSvMl0KRoMT187N2JNS6fRvVCISq_i2NyrIKrI7JEUg8Rs1SHdWnp78z1SGZD2O12faOvT2sRn2ASBRquc1BsL7L3eI6ai5pg6B8kJPP2dOfQxVdiIh8XOPfNsNPouhGLLpEGA9pg2mixy1KQ8sr1eb3d9.y1yAULrtY_8K2lP3877ksWIIaq0q4DZdJM0rEl8PukdOSiVstixP6kcc1OG_5CV64kGNK7Plg5GJmDrnukK_9V0b7M5jME9s5bsc0lx1ptEeq9.GQbBFNh0CbNM4JXm6VIDFHCIhE",
// 		"cRq": {
// 			"ru": "aHR0cHM6Ly9jZnNjaGwucGVldC53cy8=",
// 			"ra": "TW96aWxsYS81LjAgKE1hY2ludG9zaDsgSW50ZWwgTWFjIE9TIFggMTBfMTVfNykgQXBwbGVXZWJLaXQvNTM3LjM2IChLSFRNTCwgbGlrZSBHZWNrbykgQ2hyb21lLzExNS4wLjAuMCBTYWZhcmkvNTM3LjM2IC0tYXV0by1vcGVuLWRldnRvb2xzLWZvci10YWJz",
// 			"rm": "R0VU",
// 			"d": "/z2RC8Z1oxKV+Dpb+4rl6qHnfZk1xNK6p5NPO75G9kzhwn+9S7Q40DlNOwuG8C4I7RbCt+vSqwkzoolgKwhqCwiYvUmjElSfQ9DNrV/Hh+ezMIWQAennBcs0QhMUT09965TDXt9YOMLYYfKhduFSP+rKWALJbNOmX1bMODf/U2JszuA71h2uroX5yW/ju1/2XOP/7pZcZx97NZ3oPwUDjhxDE9Z3GSA7Oi4P9AYzLi18/HXYjkYubP9JQNWhaDmfBIE9NTk7wNRaDUjxzmj5CyEv1hrYH+2+9HoRsUFKq2tQHTn7dAWvlyb3PpjePSyCAOlc/ZHUKDgvp6m/+rUMnXu+LsIJiGmwE9uEurB1SJvo7VEz10n/wprpGmfSvsMkQu2EuLSBq5/r4gcTuItHzWTjIbiEp5nqSvGOmRLXQzscwgw9NyGseXJlp79Y7Klanyfze0HWXsCX9zkhgatHQiQy8HrMDlEaMS15gqylR6CTNsmLI4xE2je2R7HgP/ilUafglm1f1RUNnxIQENCnt3fY1euvVTMq6QMJFIKGq+Y=",
// 			"t": "MTcxMzc5NDA5MS43MzgwMDA=",
// 			"cT": 1713794091,
// 			"m": "A8gLgzbjm+wteWFXp9BWdL4KQ5dIeLhQe8JOkYzKPRU=",
// 			"i1": "yRRpwvdHb/i0iZa5Uyzc+Q==",
// 			"i2": "kxHsvv/emTNcGDxbGbb8/Q==",
// 			"zh": "g0oLPmGBHOOANgRh/0LsbsmhK+eLKswnGfZ3vKZJJPk=",
// 			"uh": "rVuSC3dCWTocVsZ62v58Jd4+XRPiSoVDhoenpWKoYbo=",
// 			"hh": "9SXEBZ7jq2GTlkpWURqPC0QcqE9KkBpV1ezCnFXR0aY="
// 		}
// 	}
// }
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagicBits {
    start_enc: i32,
    opcode_enc: Vec<i32>,
    // NEW_ARR: Vec<i32>,
    // JUMP_IF: Vec<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChlData {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bytecodes {
    pub init: String,
    pub main: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VMConfig {
    pub registers: HashMap<String, f64>,
    pub magic_bits: MagicBits,
    pub bytecodes: Bytecodes,
    pub chl_data: ChlData,
}
