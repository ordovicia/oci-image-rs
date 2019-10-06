use crate::Result;

pub fn convert_config(image_config: &spec::Image) -> Result<()> {
    #![allow(unused)]

    dbg!(image_config);

    let cfg = image_config.config.as_ref();
    dbg!(cfg);

    let cwd = cfg
        .and_then(|c| c.working_dir.as_ref())
        .cloned()
        .unwrap_or_default();
    dbg!(cwd);

    let env = cfg.map(|c| &c.env).cloned().unwrap_or_default();
    dbg!(env);

    let args = if let Some(ep) = cfg.map(|c| &c.entrypoint) {
        let mut args = ep.clone();
        if let Some(cmd) = cfg.map(|c| &c.cmd) {
            args.append(&mut cmd.clone());
        }
        args
    } else {
        cfg.map(|c| &c.cmd).cloned().unwrap_or_default()
    };
    dbg!(args);

    let os = cfg
        .and_then(|c| c.labels.get("os"))
        .map(|os| os.parse::<spec::descriptor::Os>().unwrap()) // TODO: no unwrap
        .unwrap_or_else(|| image_config.os.clone());
    dbg!(os);

    let architecture = cfg
        .and_then(|c| c.labels.get("architecture"))
        .map(|arch| arch.parse::<spec::descriptor::Architecture>().unwrap()) // TODO: no unwrap
        .unwrap_or_else(|| image_config.architecture.clone());
    dbg!(architecture);

    let author = cfg
        .and_then(|c| c.labels.get("author").cloned())
        .or_else(|| image_config.author.clone());
    dbg!(author);

    // TODO: created

    let stop_signal = cfg.and_then(|c| {
        c.labels
            .get("StopSignal")
            .cloned()
            .or_else(|| c.stop_signal.clone())
    });
    dbg!(stop_signal);

    // TODO: config.labels

    Ok(())
}
