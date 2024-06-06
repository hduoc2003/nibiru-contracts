use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::state::{BaseProof, ExtraProof, BASE_PROOF, EXTRA_PROOF};
use crate::tasks::create_account::verify_account_creation;
use crate::tasks::Tasks;
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult,
    Uint256,
};
use std::str::FromStr;

// instantiate the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, StdError> {
    BASE_PROOF.save(
        deps.storage,
        &BaseProof {
            r: Uint256::from_str(
                "21888242871839275222246405745257275088548364400416034343698204186575808495617",
            )
            .unwrap(),
            q: Uint256::from_str(
                "21888242871839275222246405745257275088696311157297823662689037894645226208583",
            )
            .unwrap(),
            alphax: Uint256::from_str(
                "20491192805390485299153009773594534940189261866228447918068658471970481763042",
            )
            .unwrap(),
            alphay: Uint256::from_str(
                "9383485363053290200918347156157836566562967994039712273449902621266178545958",
            )
            .unwrap(),
            betax1: Uint256::from_str(
                "4252822878758300859123897981450591353533073413197771768651442665752259397132",
            )
            .unwrap(),
            betax2: Uint256::from_str(
                "6375614351688725206403948262868962793625744043794305715222011528459656738731",
            )
            .unwrap(),
            betay1: Uint256::from_str(
                "21847035105528745403288232691147584728191162732299865338377159692350059136679",
            )
            .unwrap(),
            betay2: Uint256::from_str(
                "10505242626370262277552901082094356697409835680220590971873171140371331206856",
            )
            .unwrap(),
            gammax1: Uint256::from_str(
                "11559732032986387107991004021392285783925812861821192530917403151452391805634",
            )
            .unwrap(),
            gammax2: Uint256::from_str(
                "10857046999023057135944570762232829481370756359578518086990519993285655852781",
            )
            .unwrap(),
            gammay1: Uint256::from_str(
                "4082367875863433681332203403145435568316851327593401208105741076214120093531",
            )
            .unwrap(),
            gammay2: Uint256::from_str(
                "8495653923123431417604973247489272438418190587263600148770280649306958101930",
            )
            .unwrap(),
        },
    )?;

    EXTRA_PROOF.save(
        deps.storage,
        Tasks::CREATE_ACCOUNT,
        &ExtraProof {
            deltax1: Uint256::from_str(
                "7091163158464539332502170256123683923161169202669808516134818286225547734044",
            )
            .unwrap(),
            deltax2: Uint256::from_str(
                "15433047645566342751733967583406186363893435189454352839968398784653379235299",
            )
            .unwrap(),
            deltay1: Uint256::from_str(
                "7333219341742749029123223962146253420338323359460024818083754792608891260348",
            )
            .unwrap(),
            deltay2: Uint256::from_str(
                "9068214049224468415122916776180035185867423380498428369481232973660456913847",
            )
            .unwrap(),
            ic_x: vec![
                Uint256::from_str(
                    "18656205606996675204450018729905965229131050332687298366280649598663022404035",
                )
                .unwrap(),
                Uint256::from_str(
                    "16098888644734968208980770067728141073550489643337712182330575128413334859472",
                )
                .unwrap(),
                Uint256::from_str(
                    "21438738717490790034658384037024651100645310486501482738599590121468258393077",
                )
                .unwrap(),
                Uint256::from_str(
                    "11021958497383792777286216277277783034211777316420785049178119401706245768028",
                )
                .unwrap(),
                Uint256::from_str(
                    "1569962629038076269494370824503338914765268136296646230482407130088345282020",
                )
                .unwrap(),
                Uint256::from_str(
                    "10782257081236922070280630938858878535939066155406995059158951413286382477330",
                )
                .unwrap(),
                Uint256::from_str(
                    "6747786313226259333632873840631410383558035128104933968592214050551338916646",
                )
                .unwrap(),
            ],
            ic_y: vec![
                Uint256::from_str(
                    "12400228846190956644603566369545918126684760823954461932762418316922336827338",
                )
                .unwrap(),
                Uint256::from_str(
                    "18453209711732828678245924521144202186281101546001071095459740404354602233756",
                )
                .unwrap(),
                Uint256::from_str(
                    "20630928040478881221124023144667444171000345644852979240476344280055267284657",
                )
                .unwrap(),
                Uint256::from_str(
                    "6474816168774878855597486258106790842484026570811097544830070849066213195565",
                )
                .unwrap(),
                Uint256::from_str(
                    "8938975160247096197599564177325720690311305499097240972791709310712352291729",
                )
                .unwrap(),
                Uint256::from_str(
                    "8334732675117941231146912441611921637199486076669901919311120321238135425255",
                )
                .unwrap(),
                Uint256::from_str(
                    "4601718092389739903229053499519889330621247936254972779917214099069473791224",
                )
                .unwrap(),
            ],
        },
    )?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AccountCreation(msg) => verify_account_creation(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: Empty) -> StdResult<Binary> {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::ExecuteMsg;
    use crate::tasks::create_account::AccountCreationProof;
    use crate::types::Groth16Proof;
    use cosmwasm_std::{Addr, Empty, Uint256};
    use cw_multi_test::{App, ContractWrapper, Executor};
    use std::str::FromStr;

    #[test]
    fn test_verify_account_creation() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));
        let sender = Addr::unchecked("owner");

        let contract = app
            .instantiate_contract(code_id, sender.clone(), &Empty {}, &[], "Contract", None)
            .unwrap();

        app.execute_contract(sender, contract, &ExecuteMsg::AccountCreation(AccountCreationProof {
            relayer_hash: Uint256::from_str("2657775570588162468106059892364959818794579555689188187841520494766536623870").unwrap(),
            email_addr_pointer: Uint256::from_str("14173279942334137220153051047875524688435377838755803238289438764289764554548").unwrap(),
            account_key_commit: Uint256::from_str("4546439420997729770760366801171660106382993189994083815773060805393185157687").unwrap(),
            wallet_salt: Uint256::from_str("11645307337330358394156085775232506543267466701022933613610565396010094018508").unwrap(),
            psi_point: [
                Uint256::from_str("16190729713555891796124172902401997083132571090712478508749524955994059819467").unwrap(),
                Uint256::from_str("513507308524601422666219749690522073791387788801597937031774300495452961060").unwrap(),
            ],
            proof: Groth16Proof {
                pi_a: [
                    Uint256::from_str("15667991451084203135842214418274044746415228686680607213300256713934274025").unwrap(),
                    Uint256::from_str("16420099176460645797857596314399183692480988375105246233984376591924741730954").unwrap(),
                ],
                pi_b: [[
                    Uint256::from_str("14069042424375688912940786830845726509634682192233570256650096195882523772238").unwrap(),
                    Uint256::from_str("14766492818487001154161560044394021340925314571413688331618486581102192900723").unwrap(),
                ], [
                    Uint256::from_str("9326411115847864522892040876914271202958816187586632880905655304019455057350").unwrap(),
                    Uint256::from_str("2763427823873806702840417691613348016233722269840014653985582120557820075587").unwrap(),
                ]],
                pi_c: [
                    Uint256::from_str("8033360682341316125734842556994411540415710574394389377852915457621158475816").unwrap(),
                    Uint256::from_str("17275843055198325036192322960154758259155032495925001722114260411523773044710").unwrap(),
                ],
            },
        }), &[]).unwrap();
    }
}
