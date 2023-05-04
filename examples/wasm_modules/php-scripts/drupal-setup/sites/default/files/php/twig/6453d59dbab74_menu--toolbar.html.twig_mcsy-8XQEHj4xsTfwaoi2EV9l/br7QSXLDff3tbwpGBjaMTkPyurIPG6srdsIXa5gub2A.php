<?php

use Twig\Environment;
use Twig\Error\LoaderError;
use Twig\Error\RuntimeError;
use Twig\Extension\SandboxExtension;
use Twig\Markup;
use Twig\Sandbox\SecurityError;
use Twig\Sandbox\SecurityNotAllowedTagError;
use Twig\Sandbox\SecurityNotAllowedFilterError;
use Twig\Sandbox\SecurityNotAllowedFunctionError;
use Twig\Source;
use Twig\Template;

/* core/themes/claro/templates/navigation/menu--toolbar.html.twig */
class __TwigTemplate_7e388b5b183f0efebdd3fbe986394665 extends Template
{
    private $source;
    private $macros = [];

    public function __construct(Environment $env)
    {
        parent::__construct($env);

        $this->source = $this->getSourceContext();

        $this->parent = false;

        $this->blocks = [
        ];
        $this->sandbox = $this->env->getExtension('\Twig\Extension\SandboxExtension');
        $this->checkSecurity();
    }

    protected function doDisplay(array $context, array $blocks = [])
    {
        $macros = $this->macros;
        // line 24
        $macros["menus"] = $this->macros["menus"] = $this;
        // line 25
        echo "
";
        // line 30
        echo $this->extensions['Drupal\Core\Template\TwigExtension']->renderVar(twig_call_macro($macros["menus"], "macro_menu_links", [($context["items"] ?? null), ($context["attributes"] ?? null), 0], 30, $context, $this->getSourceContext()));
        echo "

";
    }

    // line 32
    public function macro_menu_links($__items__ = null, $__attributes__ = null, $__menu_level__ = null, ...$__varargs__)
    {
        $macros = $this->macros;
        $context = $this->env->mergeGlobals([
            "items" => $__items__,
            "attributes" => $__attributes__,
            "menu_level" => $__menu_level__,
            "varargs" => $__varargs__,
        ]);

        $blocks = [];

        ob_start(function () { return ''; });
        try {
            // line 33
            echo "  ";
            $macros["menus"] = $this;
            // line 34
            echo "  ";
            if (($context["items"] ?? null)) {
                // line 35
                echo "    ";
                if ((($context["menu_level"] ?? null) == 0)) {
                    // line 36
                    echo "      <ul";
                    echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, ($context["attributes"] ?? null), "addClass", [0 => "toolbar-menu", 1 => "claro-toolbar-menu"], "method", false, false, true, 36), 36, $this->source), "html", null, true);
                    echo ">
    ";
                } else {
                    // line 38
                    echo "      <ul class=\"toolbar-menu\">
    ";
                }
                // line 40
                echo "    ";
                $context['_parent'] = $context;
                $context['_seq'] = twig_ensure_traversable(($context["items"] ?? null));
                foreach ($context['_seq'] as $context["_key"] => $context["item"]) {
                    // line 41
                    echo "      ";
                    // line 42
                    $context["classes"] = [0 => "menu-item", 1 => ((twig_get_attribute($this->env, $this->source,                     // line 44
$context["item"], "is_expanded", [], "any", false, false, true, 44)) ? ("menu-item--expanded") : ("")), 2 => ((twig_get_attribute($this->env, $this->source,                     // line 45
$context["item"], "is_collapsed", [], "any", false, false, true, 45)) ? ("menu-item--collapsed") : ("")), 3 => ((twig_get_attribute($this->env, $this->source,                     // line 46
$context["item"], "in_active_trail", [], "any", false, false, true, 46)) ? ("menu-item--active-trail") : (""))];
                    // line 49
                    echo "      <li";
                    echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, $context["item"], "attributes", [], "any", false, false, true, 49), "addClass", [0 => ($context["classes"] ?? null)], "method", false, false, true, 49), 49, $this->source), "html", null, true);
                    echo ">
        ";
                    // line 50
                    echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->extensions['Drupal\Core\Template\TwigExtension']->getLink($this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, $context["item"], "title", [], "any", false, false, true, 50), 50, $this->source), $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, $context["item"], "url", [], "any", false, false, true, 50), 50, $this->source)), "html", null, true);
                    echo "
        ";
                    // line 51
                    if (twig_get_attribute($this->env, $this->source, $context["item"], "below", [], "any", false, false, true, 51)) {
                        // line 52
                        echo "          ";
                        echo $this->extensions['Drupal\Core\Template\TwigExtension']->renderVar(twig_call_macro($macros["menus"], "macro_menu_links", [twig_get_attribute($this->env, $this->source, $context["item"], "below", [], "any", false, false, true, 52), ($context["attributes"] ?? null), (($context["menu_level"] ?? null) + 1)], 52, $context, $this->getSourceContext()));
                        echo "
        ";
                    }
                    // line 54
                    echo "      </li>
    ";
                }
                $_parent = $context['_parent'];
                unset($context['_seq'], $context['_iterated'], $context['_key'], $context['item'], $context['_parent'], $context['loop']);
                $context = array_intersect_key($context, $_parent) + $_parent;
                // line 56
                echo "    </ul>
  ";
            }

            return ('' === $tmp = ob_get_contents()) ? '' : new Markup($tmp, $this->env->getCharset());
        } finally {
            ob_end_clean();
        }
    }

    public function getTemplateName()
    {
        return "core/themes/claro/templates/navigation/menu--toolbar.html.twig";
    }

    public function isTraitable()
    {
        return false;
    }

    public function getDebugInfo()
    {
        return array (  121 => 56,  114 => 54,  108 => 52,  106 => 51,  102 => 50,  97 => 49,  95 => 46,  94 => 45,  93 => 44,  92 => 42,  90 => 41,  85 => 40,  81 => 38,  75 => 36,  72 => 35,  69 => 34,  66 => 33,  51 => 32,  44 => 30,  41 => 25,  39 => 24,);
    }

    public function getSourceContext()
    {
        return new Source("", "core/themes/claro/templates/navigation/menu--toolbar.html.twig", "/usr/local/apache2/htdocs/drupal-10-zero/core/themes/claro/templates/navigation/menu--toolbar.html.twig");
    }
    
    public function checkSecurity()
    {
        static $tags = array("import" => 24, "macro" => 32, "if" => 34, "for" => 40, "set" => 42);
        static $filters = array("escape" => 36);
        static $functions = array("link" => 50);

        try {
            $this->sandbox->checkSecurity(
                ['import', 'macro', 'if', 'for', 'set'],
                ['escape'],
                ['link']
            );
        } catch (SecurityError $e) {
            $e->setSourceContext($this->source);

            if ($e instanceof SecurityNotAllowedTagError && isset($tags[$e->getTagName()])) {
                $e->setTemplateLine($tags[$e->getTagName()]);
            } elseif ($e instanceof SecurityNotAllowedFilterError && isset($filters[$e->getFilterName()])) {
                $e->setTemplateLine($filters[$e->getFilterName()]);
            } elseif ($e instanceof SecurityNotAllowedFunctionError && isset($functions[$e->getFunctionName()])) {
                $e->setTemplateLine($functions[$e->getFunctionName()]);
            }

            throw $e;
        }

    }
}
