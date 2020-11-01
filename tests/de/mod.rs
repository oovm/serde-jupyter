use serde_json::Result;
use sub_projects::structures::{Cell, Jupyter};

#[test]
fn cell() -> Result<()> {
    let nb: Cell = serde_json::from_str(
        r#"
{
    "cell_type": "markdown",
    "metadata": {},
    "source": [
        "<img src=\"../images/ipython_logo.png\">"
    ]
}
    "#,
    )?;
    // Do things just like with any other Rust data structure.
    println!("{:#?}", nb);

    Ok(())
}

#[test]
fn ipython() -> Result<()> {
    let nb: Jupyter = serde_json::from_str(include_str!("ipython.json"))?;
    // Do things just like with any other Rust data structure.
    println!("{:#?}", nb);

    Ok(())
}
