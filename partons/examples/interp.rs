// Load configs and download index file
use anyhow::Result;
use ndarray::array;
use partons::configs::Configs;

fn main() -> Result<()> {
    let cfg = Configs::load()?;

    let mut source = cfg.sources[0].clone();
    source.register_cache(cfg.data_path()?);
    let index = source.index()?;

    // display the first element, if non-empty
    for set in ["NNPDF40_nnlo_as_01180"] {
        //, "MSHT20nnlo_as118", "CT18NNLO"] {
        let header = index.get(set)?;
        let mut set = source.set(&header)?;
        println!("{set:#?}");
        let grid0 = set.member(0)?;
        let val = grid0
            .evaluate(
                &vec![1, 1],
                &vec![0.1, 0.2],
                &vec![10., 10.],
                &vec![4, 4, 5],
            )
            .unwrap();
        println!("{val:?}");
    }

    Ok(())
}
