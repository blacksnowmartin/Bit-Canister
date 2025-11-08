---

## 🗓️ **4-Week Hackathon Plan**

| Week                         | Goals                        | Tools & Deliverables                                                                                                                                                       |
| ---------------------------- | ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Week 1 (Oct 27 – Nov 3)**  | 🔍 Ideation + Setup          | - Choose project idea<br>- Read ICP + ckBTC docs<br>- Clone BitcoinDeFi.dev examples<br>- Draft architecture (diagram + data flow)<br>- Join Discord + ask early questions |
| **Week 2 (Nov 4 – Nov 10)**  | ⚙️ Core Development          | - Build backend (canister in Rust or Motoko)<br>- Integrate ckBTC transactions<br>- Add test BTC transfers via ICP local dev environment                                   |
| **Week 3 (Nov 11 – Nov 17)** | 💻 Frontend + Integration    | - Build React or HTML dashboard<br>- Connect canister with frontend<br>- Add user flow (deposit, timer, withdraw)                                                          |
| **Week 4 (Nov 18 – Nov 24)** | 🚀 Finalization + Submission | - Polish UI & write README<br>- Record 2–3 min demo video<br>- Submit to Encode platform<br>- Share on X & tag @EncodeClub + @dfinity                                      |

---

## ⚙️ **Tech Stack Recommendation**

| Layer           | Tools                             |
| --------------- | --------------------------------- |
| **Backend**     | ICP Canisters (Rust preferred)    |
| **Frontend**    | React + Tailwind or plain HTML/JS |
| **Integration** | ICP Bitcoin API, ckBTC, vetKeys   |
| **Testing**     | dfx local environment             |
| **Deployment**  | ICP Mainnet canister              |

---

## 🧭 **Next Steps (Today)**

1. ✅ Attend the **Kickoff (5:00 PM GMT)** and **Ideation (5:15 PM GMT)** sessions.
2. 🧠 Pick a project idea (I can help refine it after the session).
3. 💬 Connect your **Discord** to the Encode Club server.
4. 🧾 Create your project on the portal with a short description.
5. 🛠️ Set up your local ICP dev environment (`dfx` + Rust).

---

### 🧠 **1. BTC Savings Vault (Dead Man Switch)**

**Concept:**
A smart Bitcoin vault built on ICP where users deposit BTC and set a “backup recipient.”
If the user doesn’t log in for a set time, the funds auto-transfer.

**Features:**

* ckBTC integration for deposits/withdrawals
* Timer-based smart contract
* Threshold ECDSA for secure signing
* Optional encrypted “note to heir” using vetKeys

**Why it stands out:**
Combines *DeFi + inheritance + privacy*. Great storytelling angle.

---



